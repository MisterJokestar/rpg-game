use std::sync::Arc;

use tokio::sync::{mpsc, broadcast};
use tokio_util::sync::CancellationToken;

use crate::{
    db::GameRepository, 
    enemys::{
        Enemy, 
        dummy::DummyEnemy, 
        get_enemy_by_type}, 
    models::{
        Action,
        game::{Game, GameEvent},
        user::Character
    }
};

pub struct Runner {
    character: Character,
    enemy: Box<dyn Enemy>,
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
        let enemy = match get_enemy_by_type(game_state.enemy_state.enemy_type) {
            Some(e) => e,
            None => Box::new(DummyEnemy::new()),
        };
        Runner {
            character,
            enemy,
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
            // iterate through turn
            let (players_turn, current_turn) = handle_turn_order(
                &mut self.game_state,
                self.character.stats.speed,
                &self.enemy,
            );
            self.game_state.turn = current_turn;
            // check if enemy's turn or players turn
            if players_turn {
                let player_action = tokio::select! {
                    a = self.action_rx.recv() => match a {
                        Some(action) => action,
                        None => break, // Sender dropped.
                    },
                    _ = self.cancel.cancelled() => break,
                };
            } else {
                // Enemy's Turn
                let enemys_action = self.enemy.choose_action();
            }
            // update self.game_state
            // broadcast via event_tx
            // self.event_tx.send(GameEvent::TurnResolved(self.game_state.clone()))
        } // End of game loop.

        self.cleanup().await;
    }
}

// on true, player is next, on false, enemy is next.
// also returns current turn.
fn handle_turn_order(game: &mut Game, character_speed: i32, enemy: &Box<dyn Enemy>) -> (bool, i32) {
    let current_turn = game.turn;
    let players_next_turn = match game.player_state.next_turn {
        Some(turn) => turn,
        None => {current_turn + character_speed}, // Update to determine next turn based on speed.
    };
    let enemy_next_turn = match game.player_state.next_turn {
        Some(turn) => turn,
        None => {current_turn + enemy.next_turn()},
    };
    if players_next_turn <= enemy_next_turn {
            game.player_state.next_turn = Some(players_next_turn + character_speed);
            return (true, players_next_turn)
    }
    game.enemy_state.next_turn = Some(enemy_next_turn + enemy.next_turn());
    ( false, enemy_next_turn )
}

// TODO: Checklist
// 1. turn order -> if player_state or enemy_state have a next_turn == None 
//      use speed to determine next turn.
//      Set turn to lowest next turn, then complete turn.
//      Ties are determined that the player will go first.
//      DONE ! 
//
// 2. Complete action -> Actions can be three types,
//      Attack -> subtract opponents block from attack dmg, apply dmg to health.
//      Defend -> set block to number provided.
//      heal -> heals the specified number.
//
// 3. Broadcast state back to client.
