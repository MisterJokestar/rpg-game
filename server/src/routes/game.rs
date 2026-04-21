//! game.rs
//!
//! Route handlers for starting, ending and performing game actions(e.g. attack, heal, etc.).

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
    models::{Action, game::Game},
    state::{AppState, GameSession}
};

// Start Game Plan 
// 1. Create the runner 
// 2. Create the watcher (cloning the cancel token into the watcher),
// 3. Insert watcher.clone and action_tx.clone into a GameSession
// 4. Add session to state.sessions with users Uuid for reference.
// 5. tokio.spawns runner.run and watcher.run

// POST /games — builds the runner and watcher for this user, registers the session
// in AppState and spawns both background tasks
// pub async fn start_game(
//     State(state): State<Arc<AppState>>,
//     game_id: Option<Path<String>>,
// ) -> Result<(StatusCode, Json<Game>), AppError> {
// 
//     let game = match game_id { 
//         Some(Path(id)) => {
//             state.games.get_game_by_id(id.clone()).await?
//                 .ok_or_else(|| AppError::NotFound(format!("Game not found with id, '{}'", id)))
//         },
//         None => {
//             // TODO: use cloud function to create game
//         },
//     };
// 
//     // Create the runner
//     let mut runner = Runner::new(character, game.clone(), state.games.clone());
// 
//     // Make copy of the handles for the session
//     let action_tx = runner.action_tx.clone();
//     let event_tx = runner.event_tx.clone();
//     let cancel = runner.cancel.clone(); // watcher executes to stop runner
//     let snapshot = runner.snapshot.clone();
// 
//     // Create the watcher with cancel, ping, and stop
//     let ping = Arc::new(Notify::new());
//     let stop = Arc::new(Notify::new());
//     let watcher = Watcher::new(cancel, ping, stop);
// 
//     // Build the GameSession
//     let session = GameSession{
//         action_tx,
//         event_tx,
//         snapshot,
//         watcher: watcher.clone(),
//     };
// 
//     {
//         // Register the session under the game id
//         let mut sessions = state.sessions.write().await;
//         sessions.insert(game.id, session);
//     }
// 
//     // Fire runner and watcher
//     tokio::spawn(async move { runner.run().await });
//     tokio::spawn(async move { watcher.run().await });
// 
//     Ok((StatusCode::CREATED, Json(game)))
// }

// Stop Game Plan 
// 1. grab session from state.sessions by user Uuid 
// 2. session.watcher.stop.notify_one()

// POST /games/:game_id/stop — signal the watcher to cancel the runner
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

// 1. grab session from state.sessions by user Uuid 
// 2. session.watcher.ping.notify_one() (resets timer)
// 3. send action with action_tx

// POST /games/:game_id/actions — send a player action to the runner
pub async fn send_action(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<String>,
    Json(action): Json<Action>,
) -> Result<StatusCode, AppError> {
    // Grab session from state.sessions by user Uuid
    let (action_tx, ping) = {
        let sessions = state.sessions.read().await;
        let session = sessions.get(&game_id)
        .ok_or_else(|| AppError::NotFound(format!("No active session with id, '{}'", &game_id)))?;
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
        let data = serde_json::to_string(&snapshot_game).unwrap();
        Ok(Event::default().event("snapshot").data(data))
    });

    // Skip any buffered events already covered by the snapshot
    let live_stream = BroadcastStream::new(rx).filter_map(move |event| match event {
        Ok(e) if e.seq <= snapshot_seq => None,
        Ok(e) => {
            let data = serde_json::to_string(&e.event).unwrap();
            Some(Ok(Event::default().data(data)))
        }
        Err(BroadcastStreamRecvError::Lagged(n)) => {
            Some(Ok(Event::default().event("lag").data(n.to_string())))
        }
    });

    Ok(Sse::new(snapshot_event.chain(live_stream)))
}

pub async fn get_game(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Game>, AppError> {
    let id = String::from("mFVDZKECnRrhGbKaTt5e");
    let game = state.games.get_game_by_id(id.clone()).await?
        .ok_or_else(|| AppError::NotFound(format!("No game by that id '{}'", id)))?;
    Ok(Json(game))
}
