use std::sync::Arc;

use tokio::sync::mpsc;
// use tokio_util::sync::CancellationToken;

use crate::{
    db::GameRepository,
    models::{
        Action,
        game::Game,
        user::Character
    }
};

pub struct Runner {
    character: Character,
    game_state: Game,
    action_rx: mpsc::Receiver<Action>,
    pub action_tx: mpsc::Sender<Action>,
    // pub cancel: CancellationToken,
    games: Arc<dyn GameRepository>,
}

impl Runner {
    pub fn new(
        character: Character,
        game_state: Game,
        games: Arc<dyn GameRepository>
    ) -> Self {
        let (action_tx, action_rx) = mpsc::channel(1);
        Runner {
            character,
            game_state,
            action_rx,
            action_tx,
            // cancel: CancellationToken::new(),
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
            // call enemy stuff
            // update self.game_state

            let player_action = tokio::select! {
                a = self.action_rx.recv() => match a {
                    Some(action) => action,
                    None => break, // Sender dropped.
                },
                // _ = self.cancel.cancelled() => break,
            };

            // resolve players turn 
            // update self.game_state
        }

        self.cleanup().await;
    }
}
