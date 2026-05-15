use serde::{Deserialize, Serialize};

use crate::models::game::{Game, combatant::action::Target};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffects {
    before_turn: Vec<Status>,
    after_turn: Vec<Status>
}

impl StatusEffects {
    pub fn new() -> Self {
        StatusEffects {
            before_turn: Vec::new(),
            after_turn: Vec::new()
        }
    }

    fn add_status(&mut self, status_type: StatusType) {
        match status_type {
            StatusType::Before(status) => {self.before_turn.push(status);},
            StatusType::After(status) => {self.after_turn.push(status);}
        }
    }

    pub fn add_before_turn(
        &mut self,
        target: Target,
        apply: fn(Target, &mut Game)
    ) {
        self.add_status(StatusType::Before(Status { target, apply }));
    }

    pub fn add_after_turn(
        &mut self,
        target: Target,
        apply: fn(Target, &mut Game)
    ) {
        self.add_status(StatusType::After(Status { target, apply }));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusType {
    Before(Status),
    After(Status)
}

pub struct Status {
    target: Target,
    apply: fn(Target, &mut Game)
}

impl Status {
    pub fn new(
        target: Target,
        apply: fn(Target, &mut Game)
    ) -> Self {
        Status {
            target,
            apply
        }
    }

    pub fn apply_status(self, game: &mut Game) {
        self.apply(self.target, game);
    }
}
