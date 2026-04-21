use std::sync::Arc;
use tokio::sync::{mpsc, broadcast, RwLock};
use tokio_util::sync::CancellationToken;

use crate::{
    db::GameRepository,
    enemys::{
        Enemy,
        get_enemy_by_type},
    models::{
        Action,
        game::{Combatant, Game, GameEvent, SequencedEvent},
        user::Character
    }
};

// Runner is used to run a game session
pub struct Runner {
    character: Character,
    enemy: Box<dyn Enemy>,
    game_state: Game,
    action_rx: mpsc::Receiver<Action>,
    pub action_tx: mpsc::Sender<Action>,
    pub event_tx: broadcast::Sender<SequencedEvent>,
    pub cancel: CancellationToken,
    pub snapshot: Arc<RwLock<(u64, Game)>>,
    seq: u64,
    games: Arc<dyn GameRepository>,
}

impl Runner {
    // Sets up new runner
    pub fn new(
        character: Character,
        game_state: Game,
        games: Arc<dyn GameRepository>
    ) -> Self {
        // Used to recieve player actions
        let (action_tx, action_rx) = mpsc::channel(1);
        // Used to broadcast state updates (up to 16 clients can subscribe)
        let (event_tx, _) = broadcast::channel(16);
        // generates enemy off of type in game state.
        let enemy = get_enemy_by_type(game_state.enemy_state.enemy_type);
        let snapshot = Arc::new(RwLock::new((0u64, game_state.clone())));
        Runner {
            character,
            enemy,
            game_state,
            action_rx,
            action_tx,
            event_tx,
            cancel: CancellationToken::new(),
            snapshot,
            seq: 0,
            games
        }
    }

    // On cleanup records game state to database.
    pub async fn cleanup(&mut self) {
        self.seq += 1;
        let seq = self.seq;
        *self.snapshot.write().await = (seq, self.game_state.clone());
        let _ = self.event_tx.send(SequencedEvent { seq, event: GameEvent::GameStopped(self.game_state.clone()) });

        if let Err(e) = self.games.update_game_by_id(self.game_state.id.clone(), &self.game_state).await {
            tracing::error!("Failed to persist game {} on cleanup: {:?}", self.game_state.id, e);
        };
    }

    pub async fn run(&mut self){
        loop {
            if self.game_state.complete {
                break;
            }
            // iterate through turn
            let players_turn = handle_turn_order(
                &mut self.game_state,
                self.character.stats.speed,
                &self.enemy,
            );
            // check if enemy's turn or players turn
            if players_turn {
                let player_action = tokio::select! {
                    a = self.action_rx.recv() => match a {
                        Some(action) => action,
                        None => break, // Sender dropped.
                    },
                    // watcher canceled runner.
                    _ = self.cancel.cancelled() => break,
                };
                handle_turn(player_action, &mut self.game_state.player_state, &mut self.game_state.enemy_state);
                self.enemy.after_players_turn(&mut self.game_state);
            } else {
                // Enemy's Turn
                let enemys_action = self.enemy.choose_action(&mut self.game_state);
                handle_turn(enemys_action, &mut self.game_state.enemy_state, &mut self.game_state.player_state);
            }
            // broadcast via event_tx TODO: Error Handling
            self.seq += 1;
            let seq = self.seq;
            *self.snapshot.write().await = (seq, self.game_state.clone());
            let _ = self.event_tx.send(SequencedEvent { seq, event: GameEvent::TurnResolved(self.game_state.clone()) });
        } // End of game loop.
        self.cleanup().await;
    }
}

// on true, player is next, on false, enemy is next.
fn handle_turn_order(game: &mut Game, character_speed: i64, enemy: &Box<dyn Enemy>) -> bool {
    // Gets current turn, players next turn and enemys next turn.
    let current_turn = game.turn;
    let players_next_turn = match game.player_state.next_turn {
        Some(turn) => turn,
        None => {current_turn + character_speed}, // Update to determine next turn based on speed.
    };
    let enemy_next_turn = match game.player_state.next_turn {
        Some(turn) => turn,
        None => {current_turn + enemy.next_turn(game)},
    };
    // Finds out whos turn is next.
    if players_next_turn <= enemy_next_turn {
        // players turn is next, update next_turn and current turn in state.
        game.player_state.next_turn = Some(players_next_turn + character_speed);
        game.turn = players_next_turn;
        return true
    }
    // Enemys turn, update nex_turn and current turn in state.
    game.enemy_state.next_turn = Some(enemy_next_turn + enemy.next_turn(game));
    game.turn = enemy_next_turn;
    false
}

// Will handle an action.
fn handle_turn(
    action: Action, 
    attacker: &mut impl Combatant, 
    defender: &mut impl Combatant
) {
    attacker.reset_block();
    // logic for handling each action held in Combatant class.
    match action {
        Action::Attack(dmg) => {
            let dealt_dmg = defender.take_damage(dmg);
            attacker.deal_damage(dealt_dmg);
        },
        Action::Defend(block) => {
            attacker.set_block(block);
        },
        Action::Heal(heal) => {
            attacker.heal_damage(heal);
        },
        Action::None => {}
    }
}

// Handling new rounds?
//
// What is the end of a game?
