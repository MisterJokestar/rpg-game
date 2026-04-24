use crate::{
    enemys::{Enemy, EnemyType},
    models::{
        Action,
        game::{EnemyState, Game, Health}
    }
};

pub struct IronLotusEnemy {
    power: i64,
    burn: i64,
    set_message: Option<String>,
}

impl IronLotusEnemy {
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
        // speed is set to alwasy be 6
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
        // if player healed, increae burn damage
        if let Action::Heal(_) = prev_action {
            self.burn += 3;
            self.set_message = Some(String::from("Do you wish for the flames to grow?"));
        };
    }

    fn message(&mut self) -> Option<String> {
        let send = self.set_message.clone();
        self.set_message = None;
        send
    }
}
