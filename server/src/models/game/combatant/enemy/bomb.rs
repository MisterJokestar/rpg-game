//! Bomb enemy — a countdown detonator.
//!
//! The Bomb has only 1 HP and counts down 20 turns before detonating. If the
//! player attacks the Bomb before it detonates, it explodes immediately for 20
//! damage. When it detonates naturally (turn counter expires) it attacks for a
//! base power of 5, sets its own HP to 0, and is defeated.
use crate::models::game::{
    Game,
    combatant::action::{
        ActionCard,
        ActionType,
        Target
    },
};

/// A fragile enemy that detonates after 20 turns or when struck.
///
/// **Detonation mechanic:** waits 20 turns, then self-destructs with an
/// `Attack(5)` action (killing itself). If the player attacks at any point
/// before detonation, the bomb explodes immediately dealing 20 damage to the
/// player.
pub struct Bomb {
    reference: String,
    stage: i64,
    set_message: Option<String>,
}

impl Bomb {
    /// Create a new BombEnemy.
    pub fn new(reference: String) -> Self {
        Bomb {
            reference,
            stage: 0,
            set_message: None,
        }
    }

    pub fn next_turn(&mut self) -> i64 {
        if self.stage == 0 {
            self.set_message = Some(String::from("T-20 turns!"));
        }
        self.stage = 1;
        20
    }

    pub fn set_up(self, game: &mut Game) {
        game.status_effects.add_after_turn(
            Target::Enemy(Some(self.reference)),
            trigger_explosion_status
        );
    }

    pub fn choose_action(&mut self) -> ActionCard {
        // Bomb self-destructs when its turn arrives
        self.set_message = Some(String::from("KABOOM!"));
        explode_card()
    }

    pub fn message(&mut self) -> Option<String> {
        let send = self.set_message.clone();
        self.set_message = None;
        send
    }
}

fn explode_card() -> ActionCard {
    ActionCard::create_card(
        "Explosion!".into(),
        Target::AllCombatants,
        ActionType::Attack(20)
    )
}

fn trigger_explosion_status(target: Target, game: &mut Game) {
    let bomb = match target {
        Target::Enemy(Some(reference)) => {
            match game.combatants.get(&reference) {
                Some(b) => {b},
                _ => {return}
            }
        },
        _ => {return}
    };
    if bomb.health == 0 {
        game.play_card(explode_card());
    }
}
