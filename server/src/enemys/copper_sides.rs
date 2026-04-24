use crate::{
    enemys::{Enemy, EnemyType},
    models::{
        Action, 
        game::{EnemyState, Game, Health}
    }
};

pub struct CopperSidesEnemy {
    power: i64,
    defense: i64,
    set_message: Option<String>
}

impl CopperSidesEnemy {
    pub fn new() -> Self {
        CopperSidesEnemy {
            power: 1,
            defense: 6,
            set_message: None
        }
    }
}

impl Enemy for CopperSidesEnemy {
    fn get_new_state(&mut self) -> EnemyState {
        EnemyState { 
            next_turn: None, 
            state: 2, 
            health: Health { 
                current: 50, 
                max: 50
            }, 
            block: 20, 
            enemy_type: EnemyType::CopperSides
        }
    }

    fn next_turn(&mut self, state: &mut Game) -> i64 {
        // on first call, will adjust initial power depending on round,
        if state.enemy_state.state == 2 {
            self.power += state.round;
            state.enemy_state.state = 0;
        }
        8
    }

    // This will change.
    fn choose_action(&mut self, state: &mut Game) -> Action {
        // Enemy will attack on turn, then next turn will block.
        if state.enemy_state.state == 0 {
            state.enemy_state.next_turn = Some(state.turn + 1);
            state.enemy_state.state = 1;
            Action::Attack(self.power)
        } else {
            state.enemy_state.state = 0;
            self.power += 1;
            self.set_message = Some(String::from("CHARGING!"));
            Action::Defend(self.defense)
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
