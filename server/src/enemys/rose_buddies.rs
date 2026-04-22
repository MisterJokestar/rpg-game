use crate::{
    enemys::{Enemy, EnemyType}, models::{Action, game::{EnemyState, Game, Health}}
};

// Dummy enemy is a stand in enemy, 
// on its action it will do nothing.

pub struct RoseBuddiesEnemy {
    power: i64,
    defense: i64,
    speed: i64,
    set_message: Option<String>,
}

impl RoseBuddiesEnemy {
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

    // This will change.
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
        // When player attacks, rose buddies hurts player with "Thorns"
        if let Action::Attack(_) = prev_action {
            state.player_state.health.current -= 1;
            self.set_message = Some(String::from("Thorns!"));
        };
    }

    fn message(&mut self) -> Option<String> {
        self.set_message.clone()
    }
}
