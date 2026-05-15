//! Rose Buddies enemy.
//!
//! A fast attacker that cycles between attacking (twice) and healing. When the
//! player attacks, Rose Buddies retaliates with a "Thorns" proc that deals 1
//! damage back. It speeds up when below half health.
use crate::{
    models::{Action, 
        game::{
            Game,
        }
    }
};

/// A thorny, self-healing enemy that punishes attackers.
///
/// **Attack pattern:** attacks twice, then heals once (cycle repeats). When
/// below 20 HP the turn interval decreases, making it act more frequently.
pub struct RoseBuddies {
    power: i64,
    defense: i64,
    speed: i64,
    set_message: Option<String>,
}

impl RoseBuddies {
    /// Create a new RoseBuddiesEnemy with default stats.
    pub fn new() -> Self {
        RoseBuddies {
            power: 4,
            defense: 2,
            speed: 5,
            set_message: None
        }
    }

    fn next_turn(&mut self, state: &mut Game) -> i64 {
        if state.enemy_state.health.current < 20 {
            8 - self.speed
        } else {
            10 - self.speed
        }
    }

    fn choose_action(&mut self, state: &mut Game) -> Action {
        if state.enemy_state.state < 2 {
            // Base state -> attack with power
            state.enemy_state.state += 1; // attack twice then heal.
            Action::Attack(self.power)
        } else {
            // second state -> heal with defense
            state.enemy_state.state = 0;
            Action::Heal(self.defense)
        }
    }

    fn after_players_turn(&mut self, state: &mut Game, prev_action: &Action) {
        // When player attacks, Rose Buddies hurts player with "Thorns"
        if let Action::Attack(_) = prev_action {
            state.player_state.deal_damage(1);
            self.set_message = Some(String::from("Thorns!"));
        };
    }

    fn message(&mut self) -> Option<String> {
        let send = self.set_message.clone();
        self.set_message = None;
        send
    }
}
