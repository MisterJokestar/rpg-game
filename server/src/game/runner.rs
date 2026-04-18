use std::sync::Arc;

use tokio::sync::{mpsc, broadcast};
use tokio_util::sync::CancellationToken;

use crate::{
    db::GameRepository,
    models::{
        Action,
        game::{Game, GameEvent},
        user::Character
    }
};

pub struct Runner {
    character: Character,
    game_state: Game,
    action_rx: mpsc::Receiver<Action>,
    pub action_tx: mpsc::Sender<Action>,
    pub event_tx: broadcast::Sender<GameEvent>,
    pub cancel: CancellationToken,
    games: Arc<dyn GameRepository>,
}

impl Runner {
    pub fn new(
        character: Character,
        game_state: Game,
        games: Arc<dyn GameRepository>
    ) -> Self {
        let (action_tx, action_rx) = mpsc::channel(1);
        let (event_tx, _) = broadcast::channel(16);
        Runner {
            character,
            game_state,
            action_rx,
            action_tx,
            event_tx,
            cancel: CancellationToken::new(),
            games
        }
    }

    pub async fn cleanup(&mut self) {
        if let Err(e) = self.games.update_game(
            self.game_state.clone()
        ).await {
            tracing::error!("Failed to persist game state on cleanup: {}", e);
        }
    }

    pub async fn run(&mut self){
        loop {
            if self.game_state.complete {
                break;
            }
            // iterate through turn linked list 
            // check if enemy's turn or players turn

            let player_action = tokio::select! {
                a = self.action_rx.recv() => match a {
                    Some(action) => action,
                    None => break, // Sender dropped.
                },
                _ = self.cancel.cancelled() => break,
            };

            // resolve turn 
            // update self.game_state
            // broadcast via event_tx
            // self.event_tx.send(GameEvent::TurnResolved(self.game_state.clone()))
        }

        self.cleanup().await;
    }
}
