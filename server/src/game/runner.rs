//! Game loop runner.
//!
//! [`Runner`] drives the turn-based combat loop for a single active game
//! session. It is spawned as a Tokio task by
//! `start_game` and runs until the game ends, the
//! action channel is closed, or the [`CancellationToken`] is triggered by the
//! [`crate::game::watcher::Watcher`].
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

/// Async task that runs the game loop for one session.
///
/// Holds all mutable game state and the channels needed to communicate with
/// the outside world. Public fields are cloned into the [`crate::state::GameSession`]
/// record so that route handlers can interact with the running loop.
pub struct Runner {
    character: Character,
    enemy: Box<dyn Enemy>,
    game_state: Game,
    action_rx: mpsc::Receiver<Action>,
    /// Send a player action to the running loop.
    pub action_tx: mpsc::Sender<Action>,
    /// Broadcast game events to all SSE subscribers (up to 16 simultaneous
    /// listeners).
    pub event_tx: broadcast::Sender<SequencedEvent>,
    /// Cancel token shared with the [`crate::game::watcher::Watcher`]; when
    /// triggered the loop exits and calls [`Runner::cleanup`].
    pub cancel: CancellationToken,
    /// Latest `(sequence_number, game_state)` snapshot, written after every
    /// resolved turn and read by SSE stream handlers on connection.
    pub snapshot: Arc<RwLock<(u64, Game)>>,
    seq: u64,
    games: Arc<dyn GameRepository>,
}

impl Runner {
    /// Construct a new runner for `game_state` using the given `character`
    /// stats.
    ///
    /// Initialises the action and broadcast channels, resolves the current
    /// enemy from the game state, and sets the initial snapshot.
    pub fn new(
        character: Character,
        game_state: Game,
        games: Arc<dyn GameRepository>
    ) -> Self {
        // Used to receive player actions
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

    /// Persist the final game state and broadcast either a [`GameEvent::GameOver`]
    /// or [`GameEvent::GameStopped`] event before the task exits.
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

    /// Start the game loop, blocking until the game ends or is cancelled.
    ///
    /// Each iteration resolves turn order, waits for the appropriate action
    /// (player input or AI decision), applies the turn, checks win/loss
    /// conditions, then broadcasts the updated state. On exit, [`Runner::cleanup`]
    /// is called unconditionally to persist the final state.
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
            // check if enemy's turn or player's turn
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
            // broadcast via event_tx
            self.seq += 1;
            let seq = self.seq;
            *self.snapshot.write().await = (seq, self.game_state.clone());
            let _ = self.event_tx.send(SequencedEvent { seq, event: GameEvent::TurnResolved(self.game_state.clone()) });
        } // End of game loop.
        self.cleanup().await;
    }

    /// Check post-turn win/loss and round-progression conditions.
    ///
    /// Returns `true` if the game loop should exit (player died). If the
    /// enemy died, a new enemy is spawned and the round counter is incremented
    /// instead.
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
                let current_count = self.game_state.enemies_defeated.get_mut(&dead_type).unwrap();
                *current_count += 1;
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

    /// Broadcast a string message to listeners.
    ///
    /// Note: the snapshot is NOT updated for message events — they carry
    /// no state change.
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

/// Determine whose turn it is and advance the turn counter.
///
/// Returns `true` if it is the player's turn, `false` if it is the enemy's.
/// The function calculates each combatant's next scheduled turn from their
/// speed stats and advances `game.turn` to whichever is earliest.
fn handle_turn_order(game: &mut Game, character_speed: i64, enemy: &mut Box<dyn Enemy>) -> bool {
    // Gets current turn, player's next turn and enemy's next turn.
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
    // Finds out whose turn is next.
    if players_next_turn <= enemy_next_turn {
        // player's turn is next, update next_turn and current turn in state.
        game.player_state.next_turn = Some(players_next_turn + character_turn_increment);
        game.turn = players_next_turn;
        return true
    }
    // Enemy's turn, update next_turn and current turn in state.
    game.enemy_state.next_turn = Some(enemy_next_turn + enemy.next_turn(game));
    game.turn = enemy_next_turn;
    false
}

/// Apply a single combat action from `attacker` to `defender`.
///
/// Resets the attacker's block at the start of the turn, then dispatches to
/// the appropriate [`Combatant`] method based on the action variant. Damage
/// amounts are randomised within a range derived from the stat payload.
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
