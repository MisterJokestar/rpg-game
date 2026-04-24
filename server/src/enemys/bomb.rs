use crate::{
    enemys::{Enemy, EnemyType}, models::{Action, game::{Combatant, EnemyState, Game, Health}}
};

// Dummy enemy is a stand in enemy, 
// on its action it will do nothing.

pub struct BombEnemy {
    set_message: Option<String>,
}

impl BombEnemy {
    pub fn new() -> Self {
        BombEnemy { 
            set_message: None,
        }
    }
}

impl Enemy for BombEnemy {
    fn get_new_state(&mut self) -> EnemyState {
        EnemyState { 
            next_turn: None, 
            state: 0, 
            health: Health { 
                current: 1, 
                max: 1
            }, 
            block: 0, 
            enemy_type: EnemyType::Bomb
        }
    }

    fn next_turn(&mut self, state: &mut Game) -> i64 {
        _ = state;
        self.set_message = Some(String::from("T-20 turns!"));
        20
    }

    // This will change.
    fn choose_action(&mut self, state: &mut Game) -> Action {
        state.enemy_state.health.current = 0;
        self.set_message = Some(String::from("KABOOM!"));
        Action::Attack(5)
    }

    fn after_players_turn(&mut self, state: &mut Game, prev_action: &Action) {
        if let Action::Attack(_) = prev_action {
            self.set_message = Some(String::from("KABOOM!"));
            state.player_state.deal_damage(20);
        };
        
    }

    fn message(&mut self) -> Option<String> {
        let send = self.set_message.clone();
        self.set_message = None;
        send
    }
}
