use serde::{Deserialize, Serialize};

use crate::models::game::combatant::action::ActionDeck;

pub mod fighter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hero {
    pub class: HeroClass,
    pub deck: ActionDeck
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeroClass {
    Fighter
}

impl Hero {
    fn new(class: HeroClass) -> Self {
        Hero {
            class,
            deck: class.create_base_deck(),
        }
    }
}

impl HeroClass {
    fn create_base_deck(self) -> ActionDeck {
        match self {
            Self::Fighter => {fighter::get_fighter_deck()}
        }
    }
}
