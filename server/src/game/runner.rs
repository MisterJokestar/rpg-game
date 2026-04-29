use std::{collections::hash_map::Entry, sync::Arc};
use rand::random_range;
use tokio::sync::{mpsc, broadcast, RwLock};
use tokio_util::sync::CancellationToken;

use crate::{
    db::GameRepository,
    enemys::{
        Enemy,
        get_enemy_by_type, get_random_enemy},
    models::{
        Action,
        game::{Combatant, Game, GameEvent, SequencedEvent},
        character::Character
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
        let event = if self.game_state.complete {
            GameEvent::GameOver(self.game_state.clone())
        } else {
            GameEvent::GameStopped(self.game_state.clone())
        };
        let _ = self.event_tx.send(SequencedEvent { seq, event });

        if let Err(e) = self.games.update_game(self.game_state.id.clone(), &self.game_state).await {
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
                &mut self.enemy,
            );
            if players_turn {
                tracing::debug!(turn = self.game_state.turn, "player's turn");
            } else {
                tracing::debug!(turn = self.game_state.turn, "enemy's turn");
            }
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
                handle_turn(&player_action, &mut self.game_state.player_state, &mut self.game_state.enemy_state);
                self.enemy.after_players_turn(&mut self.game_state, &player_action);
            } else {
                // Enemy's Turn
                let enemys_action = self.enemy.choose_action(&mut self.game_state);
                handle_turn(&enemys_action, &mut self.game_state.enemy_state, &mut self.game_state.player_state);
            }
            tracing::debug!(
                player_hp = self.game_state.player_state.health.current,
                enemy_hp = self.game_state.enemy_state.health.current,
                "turn resolved"
            );
            // if player is dead -> exit loop.
            if self.handle_after_turn() {break};
            // broadcast via event_tx TODO: Error Handling
            self.seq += 1;
            let seq = self.seq;
            *self.snapshot.write().await = (seq, self.game_state.clone());
            let _ = self.event_tx.send(SequencedEvent { seq, event: GameEvent::TurnResolved(self.game_state.clone()) });
        } // End of game loop.
        self.cleanup().await;
    }

    fn handle_after_turn(&mut self) -> bool {
        if let Some(m) = self.enemy.message() {
            self.broadcast_message(m);
        }
        // Check if player or enemy died
        if self.game_state.player_state.health.current == 0 {
            self.game_state.complete = true;
            self.game_state.win = Some(self.game_state.round > 3);
            return true
        }
        if self.game_state.enemy_state.health.current == 0 {
            let dead_type = self.game_state.enemy_state.enemy_type;
            if let Entry::Vacant(e) = self.game_state.enemies_defeated.entry(dead_type) {
                e.insert(1);
            } else {
                let curent_count = self.game_state.enemies_defeated.get_mut(&dead_type).unwrap();
                *curent_count += 1;
            }
            // Increment round generate new enemy.
            self.game_state.round += 1;
            self.game_state.turn = 0;
            self.game_state.player_state.next_turn = None;
            self.enemy = get_random_enemy();
            self.game_state.enemy_state = self.enemy.get_new_state();
        }
        false
    }

    // Broadcast a string message to listeners.
    fn broadcast_message(&mut self, message: String) {
        self.seq += 1;
        let seq = self.seq;
        // Note: snapshot is NOT updated here
        let _ = self.event_tx.send(SequencedEvent {
            seq,
            event: GameEvent::GameMessage(message),
        });
    }
}

// on true, player is next, on false, enemy is next.
fn handle_turn_order(game: &mut Game, character_speed: i64, enemy: &mut Box<dyn Enemy>) -> bool {
    // Gets current turn, players next turn and enemys next turn.
    let character_turn_increment = (19 - character_speed) / 3;
    let current_turn = game.turn;
    let players_next_turn = match game.player_state.next_turn {
        Some(turn) => turn,
        None => {
            let next = current_turn + character_turn_increment;
            game.player_state.next_turn = Some(next);
            next
        }, // Update to determine next turn based on speed.
    };
    let enemy_next_turn = match game.enemy_state.next_turn {
        Some(turn) => turn,
        None => {
            let next = current_turn + enemy.next_turn(game);
            game.enemy_state.next_turn = Some(next);
            next
        }, // Update to determine next turn based on speed.
    };
    tracing::debug!(
        players_next = players_next_turn,
        enemy_next = enemy_next_turn,
        current = current_turn,
        "turn order"
    );
    // Finds out whos turn is next.
    if players_next_turn <= enemy_next_turn {
        // players turn is next, update next_turn and current turn in state.
        game.player_state.next_turn = Some(players_next_turn + character_turn_increment);
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
    action: &Action, 
    attacker: &mut impl Combatant, 
    defender: &mut impl Combatant
) {
    attacker.reset_block();
    // logic for handling each action held in Combatant class.
    match action {
        Action::Attack(power) => {
            let dmg = random_range(2*power..5*power);
            let dealt_dmg = defender.take_damage(dmg);
            attacker.deal_damage(dealt_dmg);
        },
        Action::Defend(defense) => {
            let block = random_range(3*defense..5*defense);
            attacker.set_block(block);
        },
        Action::Heal(defense) => {
            let heal = random_range(2*defense..4*defense);
            attacker.heal_damage(heal);
        },
        Action::None => {}
    }
}
