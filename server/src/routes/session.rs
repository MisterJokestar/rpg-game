//! Game session route handlers.
//!
//! These handlers manage the lifecycle of a live, in-memory game session:
//!
//! - `POST /session/:game_id`         — start a session (authenticated).
//! - `POST /session/:game_id/stop`    — stop a session (authenticated).
//! - `POST /session/:game_id/action`  — send a player action (authenticated).
//! - `GET  /session/:game_id/stream`  — subscribe to SSE events (public).
//!
//! Starting a session spawns two Tokio tasks: a [`Runner`] that drives the
//! game loop, and a [`Watcher`] that cancels the runner after 15 minutes of
//! inactivity or when an explicit stop is requested.
use std::{
    convert::Infallible,
    sync::Arc
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::sse::{Event, Sse},
    Json
};
use tokio_stream::{
    StreamExt,
    wrappers::{
        BroadcastStream,
        errors::BroadcastStreamRecvError
}};
use futures::{Stream, stream};
use tokio::sync::Notify;
use crate::{
    error::AppError,
    game::{runner::Runner, watcher::Watcher},
    models::{Action, game::{GameEventResponse, GameResponse}},
    state::{AppState, GameSession}
};

/// `POST /session/:game_id` — start a live game session for the given game.
///
/// Loads the game from the database, constructs a [`Runner`] and [`Watcher`],
/// registers the session in [`AppState`], then spawns both tasks. Returns the
/// current game state so the client has an initial snapshot.
///
/// # Errors
///
/// - [`AppError::NotFound`] if the game or its associated character cannot be
///   found.
/// - [`AppError::Database`] if the database reads fail.
pub async fn start_game(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<String>,
) -> Result<(StatusCode, Json<GameResponse>), AppError> {
    let mut game = state.games.get_game(game_id.clone()).await?
        .ok_or_else(|| AppError::NotFound(format!("Game not found with id, '{}'", game_id)))?;

    if game.player_state.health.max == 0 {game.set_up();}

    let user_id = game.player_state.player_id.clone();
    let character_id = game.player_state.character_id.clone();

    let character = state.characters.get_character(character_id.clone()).await?
        .ok_or_else(|| AppError::NotFound(format!("User or Character not found, '{}' '{}'", user_id, character_id)))?;

    // Create the runner
    let mut runner = Runner::new(character, game.clone(), state.games.clone());

    // Make copy of the handles for the session
    let action_tx = runner.action_tx.clone();
    let event_tx = runner.event_tx.clone();
    let cancel = runner.cancel.clone(); // watcher executes to stop runner
    let snapshot = runner.snapshot.clone();

    // Create the watcher with cancel, ping, and stop
    let ping = Arc::new(Notify::new());
    let stop = Arc::new(Notify::new());
    let watcher = Watcher::new(cancel, ping, stop);

    // Build the GameSession
    let session = GameSession{
        action_tx,
        event_tx,
        snapshot,
        watcher: watcher.clone(),
    };

    {
        // Register the session under the game id
        let mut sessions = state.sessions.write().await;
        sessions.insert(game.id.clone(), session);
    }

    // Fire runner and watcher
    tokio::spawn(async move { runner.run().await });
    tokio::spawn(async move { watcher.run().await });

    Ok((StatusCode::CREATED, Json(GameResponse::from(game))))
}

/// `POST /session/:game_id/stop` — gracefully stop an active session.
///
/// Removes the session from [`AppState`] and signals the watcher to cancel
/// the runner, which will persist the game state before exiting.
///
/// # Errors
///
/// - [`AppError::NotFound`] if there is no active session for the given game
///   ID.
pub async fn stop_game(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<String>,
) -> Result<StatusCode,AppError> {
    // Pull the session out of the map so no other request can use it
    let session = {
        let mut sessions = state.sessions.write().await;
        sessions.remove(&game_id)
        .ok_or_else(|| AppError::NotFound(format!("No active session with id, '{}'", &game_id)))?

    };
    // Notify the watcher which will cause cancel_runner to fire and then cleanup
    session.watcher.stop.notify_one();
    Ok(StatusCode::NO_CONTENT)
}

/// `POST /session/:game_id/action` — submit a player action for the current
/// turn.
///
/// Resets the watcher's idle timer (keeping the session alive) and forwards
/// the action to the runner via the session's action channel.
///
/// # Errors
///
/// - [`AppError::NotFound`] if there is no active session for the given game
///   ID.
/// - [`AppError::Internal`] if the action channel has been closed
///   unexpectedly.
pub async fn send_action(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<String>,
    Json(action): Json<Action>,
) -> Result<StatusCode, AppError> {
    // Grab session from state.sessions by game id
    let (action_tx, ping) = {
        let sessions = state.sessions.read().await;
        let session = sessions.get(&game_id)
        .ok_or_else(|| AppError::NotFound(format!("No active session with id, '{}'", &game_id)))?;
        (session.action_tx.clone(), session.watcher.ping.clone())
    };
    // Reset the watcher's idle timer; the game is still active
    ping.notify_one();
    // Send the action with action_tx
    action_tx.send(action).await
        .map_err(|err| AppError::Internal(format!("Failed to send action: {}", err)))?;
    Ok(StatusCode::ACCEPTED)
}

/// `GET /session/:game_id/stream` — subscribe to a Server-Sent Events stream
/// of game updates.
///
/// On connection the handler immediately emits a `"snapshot"` event containing
/// the current game state, then forwards all subsequent [`crate::models::game::SequencedEvent`]s
/// from the broadcast channel. Events with a sequence number at or below the
/// snapshot's sequence number are dropped to avoid duplicates.
///
/// A `"lag"` event is emitted if the client falls too far behind the broadcast
/// buffer.
///
/// # Errors
///
/// - [`AppError::NotFound`] if there is no active session for the given game
///   ID.
pub async fn game_stream(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<String>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    // Subscribe first so no events are missed, then clone the snapshot Arc
    let (rx, snapshot_arc) = {
        let sessions = state.sessions.read().await;
        let session = sessions.get(&game_id)
            .ok_or_else(|| AppError::NotFound(format!("No active session for user '{}'", &game_id)))?;
        (session.event_tx.subscribe(), session.snapshot.clone())
    };

    // Read snapshot after releasing the sessions lock
    let (snapshot_seq, snapshot_game) = snapshot_arc.read().await.clone();

    // Send the current state immediately as the first event
    let snapshot_event = stream::once(async move {
        let data = serde_json::to_string(&GameResponse::from(snapshot_game)).unwrap();
        Ok(Event::default().event("snapshot").data(data))
    });

    // Skip any buffered events already covered by the snapshot
    let live_stream = BroadcastStream::new(rx).filter_map(move |event| match event {
        Ok(e) if e.seq <= snapshot_seq => None,
        Ok(e) => {
            let data = serde_json::to_string(&GameEventResponse::from(e.event)).unwrap();
            Some(Ok(Event::default().data(data)))
        }
        Err(BroadcastStreamRecvError::Lagged(n)) => {
            Some(Ok(Event::default().event("lag").data(n.to_string())))
        }
    });

    Ok(Sse::new(snapshot_event.chain(live_stream)))
}
