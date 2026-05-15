use mongodb::action::Action;
use serde::{Deserialize, Serialize};

use crate::models::game::combatant::Combatant;

mod block;
mod attack;
mod heal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDeck {
    total_deck: Vec<ActionCard>,
    in_hand: Vec<ActionCard>,
    draw_pile: Vec<ActionCard>,
    discarded: Vec<ActionCard>,
    destroyed: Vec<ActionCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionCard {
    title: String,
    actions: Vec<Action>
}

pub struct Action {
    pub target: Target,
    pub action_type: ActionType
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Target {
    CombatantSelf,
    Hero(Option<String>),
    Enemy(Option<String>),
    AllEnemies,
    AllHeros,
    AllCombatants
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    NoAction,
    Block(i64),
    Attack(i64),
    Heal(i64),
}

impl ActionType {
    fn complete_action(self, targets: Vec<&mut Combatant>) {
        targets.iter().for_each(|target| { 
            match self {
                Self::Block(def) => {block::basic_block(&mut target.health, def);},
                Self::Attack(atk) => {attack::basic_attack(&mut target.health, atk);},
                Self::Heal(heal) => {heal::basic_heal(&mut target.health, heal);},
                Self::NoAction => {}
            };
        });
    }
}

impl ActionCard {
    pub fn create_card(
        title: String,
        target:Target,
        action_type: ActionType
    ) -> Self {
        let mut actions: Vec<Action> = Vec::new();
        actions.push(Action {
            target,
            action_type
        });
        ActionCard { 
            title,
            actions
        }
    }

    pub fn add_action(
        &mut self,
        target: Target,
        action_type: ActionType
    ) {
        self.actions.push(Action { target, action_type });
    }

    pub fn get_actions(self) -> Vec<Action> {
        self.actions.iter().clone().collect()
    }
}

impl ActionDeck {
    pub fn new(total_deck: Vec<ActionCard>) -> Self {
        ActionDeck {
            total_deck,
            in_hand: Vec::new(),
            draw_pile: Vec::new(),
            discarded: Vec::new(),
            destroyed: Vec::new()
        }
    }
}

pub fn no_action_card() -> ActionCard {
    ActionCard::create_card(
        "No Action".into(),
        Target::CombatantSelf,
        ActionType::NoAction,
    )
}
