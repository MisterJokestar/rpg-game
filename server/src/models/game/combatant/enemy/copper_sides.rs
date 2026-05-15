//! Copper Sides enemy.
//!
//! A tanky enemy with a strict attack/defend cycle. On its defend turn it
//! emits a "CHARGING!" message and its power permanently increases by 1,
//! making each subsequent attack hit harder. Power also scales with the
//! current round, so it is stronger in later rounds.
use crate::models::game::{combatant::action::{ActionCard, ActionType, Target}};


/// A heavily-armoured enemy that alternates attacking and defending.
///
/// **Attack/defend cycle:** attacks, then defends (gaining +1 power each
/// cycle). Initial power is boosted by the current round number, making this
/// enemy more dangerous in later rounds.
pub struct CopperSides {
    stage: i64,
    power: i64,
    defense: i64,
    set_message: Option<String>
}

impl CopperSides {
    /// Create a new CopperSidesEnemy with default stats.
    pub fn new() -> Self {
        CopperSides {
            stage: 1,
            power: 5,
            defense: 15,
            set_message: None
        }
    }

    fn next_turn(self) -> i64 {
        if self.stage == 1 {8} else {1}
    }

    fn choose_action(&mut self) -> ActionCard {
        // Enemy attacks on state 0, then defends and charges on state 1
        if self.stage == 0 {
            self.stage = 1;
            self.attack_card()
        } else {
            self.stage = 0;
            self.power += 5;
            self.set_message = Some(String::from("CHARGING!"));
            self.defend_card()
        }
    }

    fn message(&mut self) -> Option<String> {
        let send = self.set_message.clone();
        self.set_message = None;
        send
    }

    fn attack_card(self) -> ActionCard {
        ActionCard::create_card(
            "Pummel!".into(),
            Target::AllHeros,
            ActionType::Attack(self.power)
        )
    }

    fn defend_card(self) -> ActionCard {
        ActionCard::create_card(
            "Charging!".into(),
            Target::CombatantSelf,
            ActionType::Block(self.defense)
        )
    }
}
