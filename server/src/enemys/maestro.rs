/**
 * Maestro Enemy: enemy that uses ammo, and as they get closer to their final round they get stronger
 * @authors: Sam Plemmons and James Wall
 */

use crate::{
    enemys::{Enemy, EnemyType},
    models::{
        Action, 
        game::{EnemyState, Game, Health}
    }
};

pub struct MaestroEnemy {
    power: i64,
    ammo: i64,
    defense: i64,
    speed: i64,
    set_message: Option<String>
}

impl MaestroEnemy {
    pub fn new() -> Self {
        MaestroEnemy {
            power: 8,
            ammo: 6,
            defense: 0,
            speed: 4,
            set_message: None
        }
    }
}

impl Enemy for MaestroEnemy {
    fn get_new_state(&mut self) -> EnemyState {
        EnemyState { 
            next_turn: None, 
            state: 0, 
            health: Health { 
                current: 30, 
                max: 30
            }, 
            block: 0, 
            enemy_type: EnemyType::Maestro
        }
    }

    fn next_turn(&mut self, state: &mut Game) -> i64 {
        // they get faster as their health is lower (or at least that's the goal)
        if state.enemy_state.health.current < 10 {
            6 - self.speed
        } else if state.enemy_state.health.current < 20 {
            8 - self.speed
        } else {
            10 - self.speed
        }
    }

    // This will change.
    fn choose_action(&mut self, state: &mut Game) -> Action {
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
        // reload
        } else {
            self.ammo = 6;
            self.set_message = Some(String::from("Reload!"));
            Action::None
        }
    }

    fn after_players_turn(&mut self, state: &mut Game, prev_action: &Action) {
        _ = state;
        _ = prev_action;
    }

    fn message(&mut self) -> Option<String> {
        let send = self.set_message.clone();
        self.set_message = None;
        send
    }
}
