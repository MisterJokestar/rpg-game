//! game.rs
//!
//! Route handlers for starting, ending and performing game actions(e.g. attack, heal, etc.).

use std::{convert::Infallible, sync::Arc};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::sse::{Event, Sse},
    Json
};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use uuid::Uuid;
use futures::Stream;
use tokio::sync::Notify;
use crate::{
    error::AppError,
    game::{runner::Runner, watcher::Watcher},
    models::{Action, game::{Game, NewGameRequest}},
    state::{AppState, GameSession}
};

//TODO: ALL

// Start Game Plan 
// 1. Create the runner 
// 2. Create the watcher (cloning the cancel token into the watcher),
// 3. Insert watcher.clone and action_tx.clone into a GameSession
// 4. Add session to state.sessions with users Uuid for reference.
// 5. tokio.spawns runner.run and watcher.run

// POST /games — builds the runner and watcher for this user, registers the session
// in AppState and spawns both background tasks
pub async fn start_game(
    State(state): State<Arc<AppState>>,
    Json(body): Json<NewGameRequest>,
) -> Result<(StatusCode, Json<Game>), AppError> {

    // Copy player_id and character_id since Game takes ownership
    let player_id = body.player_id;
    let character_id = body.character_id;

    // Verify the user exists and owns the requested character
    let user = state.users.get_user_by_id(player_id).await?
        .ok_or_else(|| AppError::NotFound(format!("Username '{}' not found", &player_id)))?;
    let character = user.characters.iter()
        .find(|c| c.id == character_id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("Character '{}' not found", &character_id)))?;

    // Persist a fresh Game row so the runner has something to update/cleanup
    let game = state.games.create_game(Game::new(body)).await?;
    // Create the runner
    let mut runner = Runner::new(character, game.clone(), state.games.clone());

    // Make copy of the handles for the session
    let action_tx = runner.action_tx.clone();
    let event_tx = runner.event_tx.clone();
    let cancel = runner.cancel.clone(); // watcher executes to stop runner

    // Create the watcher with cancel, ping, and stop
    let ping = Arc::new(Notify::new());
    let stop = Arc::new(Notify::new());
    let watcher = Watcher::new(cancel, ping, stop);

    // Build the GameSession
    let session = GameSession{
        action_tx,
        event_tx,
        watcher: watcher.clone(),
    };

    {
        // Register the session under the users id
        let mut sessions = state.sessions.write().await;
        sessions.insert(player_id, session);
    }

    // Fire runner and watcher
    tokio::spawn(async move { runner.run().await });
    tokio::spawn(async move { watcher.run().await });

    Ok((StatusCode::CREATED, Json(game)))
}

// Stop Game Plan 
// 1. grab session from state.sessions by user Uuid 
// 2. session.watcher.stop.notify_one()

// POST /games/:user_id/stop — signal the watcher to cancel the runner
pub async fn stop_game(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode,AppError> {
    // Pull the session out of the map so no other request can use it
    let session = {
        let mut sessions = state.sessions.write().await;
        sessions.remove(&user_id)
        .ok_or_else(|| AppError::NotFound(format!("No active session for user '{}'", &user_id)))?

    };
    // Notify the watcher which will cause cancel_runner to fire and then cleanup
    session.watcher.stop.notify_one();
    Ok(StatusCode::NO_CONTENT)
}

// 1. grab session from state.sessions by user Uuid 
// 2. session.watcher.ping.notify_one() (resets timer)
// 3. send action with action_tx

// POST /games/:user_id/actions — send a player action to the runner
pub async fn send_action(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
    Json(action): Json<Action>,
) -> Result<StatusCode, AppError> {
    // Grab session from state.sessions by user Uuid
    let (action_tx, ping) = {
        let sessions = state.sessions.read().await;
        let session = sessions.get(&user_id)
        .ok_or_else(|| AppError::NotFound(format!("No active session for user '{}'", &user_id)))?;
        (session.action_tx.clone(), session.watcher.ping.clone())
    };
    // Reset the watchers idle timer, game is still active
    ping.notify_one();
    // Send the action with action_tx
    action_tx.send(action).await
        .map_err(|err| AppError::Internal(format!("Failed to send action: {}", err)))?;
    Ok(StatusCode::ACCEPTED)
}

// SSE Endpoint
// This is to register the client as a listener for game updates
pub async fn game_stream(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = {
        let sessions = state.sessions.read().await;
        // TODO: Error handling for if user does not have a session running.
        sessions.get(&user_id).unwrap().event_tx.subscribe()
    }; // registers client to listen for updates

    let stream = BroadcastStream::new(rx).map(|event| {
        // TODO: Error handling for Err(BroadcastStreamRecvError::Lagged)
        // This happens if the reciever falls behind.
        let data = serde_json::to_string(&event.unwrap()).unwrap();
        Ok(Event::default().data(data))
    });

    Sse::new(stream)
}
