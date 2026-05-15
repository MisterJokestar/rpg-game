//! Maestro enemy — an ammo-based ranged attacker.
//!
//! Maestro fires up to 6 shots before needing to reload. Each shot is
//! slightly stronger than the last (power increases as ammo decreases). The
//! final shot deals double power damage. Reloading uses a *negative* defense
//! value, which means the "defend" action actually costs Maestro HP instead of
//! gaining block — representing the vulnerability of reloading. Maestro also
//! speeds up as its HP falls.
//!
//! Authors: Sam Plemmons
use crate::{
    models::{Action, 
        game::{
            Game,
        }
    }
};

/// A gunslinging enemy that grows more dangerous as it approaches its last
/// shot.
///
/// **Ammo mechanic:** starts with 6 shots. Each shot reduces ammo by 1 and
/// deals `power - ammo` damage (so later shots hit harder). When ammo reaches
/// 1 the final "super shot" fires at `power * 2`. When ammo hits 0 Maestro
/// reloads (defends with a negative value) and the cycle resets.
pub struct Maestro {
    power: i64,
    ammo: i64,
    defense: i64,
    speed: i64,
    set_message: Option<String>
}

impl Maestro {
    /// Create a new MaestroEnemy with default stats.
    pub fn new() -> Self {
        Maestro {
            power: 8,
            ammo: 6,
            defense: -5,
            speed: 4,
            set_message: None
        }
    }

    fn next_turn(&mut self, state: &mut Game) -> i64 {
        // Gets faster as health drops
        if state.enemy_state.health.current < 10 {
            6 - self.speed
        } else if state.enemy_state.health.current < 20 {
            8 - self.speed
        } else {
            10 - self.speed
        }
    }

    fn choose_action(&mut self) -> Action {
        // if they still have ammo, use basic attack
        if self.ammo > 1 {
            self.ammo -= 1; // reduce ammo count
            self.set_message = Some(format!("{} Rounds Left!", self.ammo));
            Action::Attack(self.power - self.ammo)
        // super attack with final round
        } else if self.ammo == 1 {
            self.ammo -= 1;
            self.set_message = Some(String::from("This one's bound finish the fight!"));
            Action::Attack(self.power * 2)
        // reload but takes more damage
        } else {
            self.ammo = 6;
            self.set_message = Some(String::from("Reload!"));
            Action::Defend(self.defense)
        }
    }

    fn message(&mut self) -> Option<String> {
        let send = self.set_message.clone();
        self.set_message = None;
        send
    }
}
