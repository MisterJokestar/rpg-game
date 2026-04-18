//! game.rs
//!
//! Route handlers for starting, ending and performing game actions(e.g. attack, heal, etc.).

use axum::{
    extract::{Path, State},
    response::sse::{Event, Sse},
};
use std::{convert::Infallible, sync::Arc};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use uuid::Uuid;
use futures::Stream;
use crate::AppState;

//TODO: ALL

// Start Game Plan 
// 1. Create the runner 
// 2. Create the watcher (cloning the cancel token into the watcher),
// 3. Insert watcher.clone and action_tx.clone into a GameSession
// 4. Add session to state.sessions with users Uuid for reference.
// 5. tokio.spawns runner.run and watcher.run 

// Stop Game Plan 
// 1. grab session from state.sessions by user Uuid 
// 2. session.watcher.stop.notify_one()

// Send Action Plan 
// 1. grab session from state.sessions by user Uuid 
// 2. session.watcher.ping.notify_one() (resets timer)
// 3. send action with action_tx 
//
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
