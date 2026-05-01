//! Iron Lotus enemy — a burn-stacking attacker.
//!
//! Iron Lotus applies passive burn damage to the player at the end of every
//! turn. Burn stacks increase each time Iron Lotus attacks (and ramp up faster
//! below half health). If the player heals, burn increases by an additional 3,
//! punishing recovery. Once per encounter, when Iron Lotus's HP falls below
//! the current burn value, it heals itself and the player simultaneously while
//! announcing a narrative message.
//!
//! Authors: Sam Plemmons and James Wall
use crate::{
    enemys::{Enemy, EnemyType},
    models::{
        Action,
        game::{EnemyState, Game, Health}
    }
};

/// An enemy that builds escalating burn damage over time.
///
/// **Burn mechanic:** `burn` starts at 0 and increases by 1 each time Iron
/// Lotus attacks (+2 per attack below half health). At the end of every
/// player turn, `burn` HP is dealt to the player. Healing increases `burn`
/// by 3. Speed is fixed at a turn interval of 6.
pub struct IronLotusEnemy {
    power: i64,
    burn: i64,
    set_message: Option<String>,
}

impl IronLotusEnemy {
    /// Create a new IronLotusEnemy with zero initial burn.
    pub fn new() -> Self {
        IronLotusEnemy {
            power: 4,
            burn: 0,
            set_message: None
        }
    }
}

impl Enemy for IronLotusEnemy {
    fn get_new_state(&mut self) -> EnemyState {
        EnemyState {
            next_turn: None,
            state: 1,
            health: Health {
                current: 40,
                max: 40
            },
            block: 0,
            enemy_type: EnemyType::IronLotus
        }
    }

    fn next_turn(&mut self, state: &mut Game) -> i64 {
        _ = state;
        // speed is always 6
        6
    }

    fn choose_action(&mut self, state: &mut Game) -> Action {
        // heal self and player to keep the flames growing (only once)
        if state.enemy_state.state == 1 && state.enemy_state.health.current < self.burn {
            state.enemy_state.state -= 1;
            self.set_message = Some(String::from("May you overcome the searing flames!"));
            state.player_state.health.current += self.burn;
            Action::Heal(self.burn)
        // basic attack which builds up burn
        } else {
            self.burn += 1;
            // ramp up burn when below half health
            if state.enemy_state.health.current < (state.enemy_state.health.max / 2) {
                self.burn += 1;
            }
            Action::Attack(self.power)
        }
    }

    fn after_players_turn(&mut self, state: &mut Game, prev_action: &Action) {
        // At end of turn, deal burn damage to player
        state.player_state.health.current -= self.burn;
        // if player healed, increase burn damage
        if let Action::Heal(_) = prev_action {
            self.burn += 3;
            self.set_message = Some(String::from("Do you also wish for the flames to grow?"));
        };
    }

    fn message(&mut self) -> Option<String> {
        let send = self.set_message.clone();
        self.set_message = None;
        send
    }
}
