//! Rose Buddies enemy.
//!
//! A fast attacker that cycles between attacking (twice) and healing. When the
//! player attacks, Rose Buddies retaliates with a "Thorns" proc that deals 1
//! damage back. It speeds up when below half health.
use crate::{
    enemys::{Enemy, EnemyType},
    models::{
        Action,
        game::{Combatant, EnemyState, Game, Health}
    }
};

/// A thorny, self-healing enemy that punishes attackers.
///
/// **Attack pattern:** attacks twice, then heals once (cycle repeats). When
/// below 20 HP the turn interval decreases, making it act more frequently.
pub struct RoseBuddiesEnemy {
    power: i64,
    defense: i64,
    speed: i64,
    set_message: Option<String>,
}

impl RoseBuddiesEnemy {
    /// Create a new RoseBuddiesEnemy with default stats.
    pub fn new() -> Self {
        RoseBuddiesEnemy {
            power: 4,
            defense: 2,
            speed: 5,
            set_message: None
        }
    }
}

impl Enemy for RoseBuddiesEnemy {
    fn get_new_state(&mut self) -> EnemyState {
        EnemyState {
            next_turn: None,
            state: 0,
            health: Health {
                current: 40,
                max: 40
            },
            block: 0,
            enemy_type: EnemyType::RoseBuddies
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
