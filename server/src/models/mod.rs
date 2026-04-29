use serde::Deserialize;

pub mod user;
pub mod character;
pub mod game;

#[derive(Debug, Clone, Deserialize)]
pub enum Action {
    Attack(i64),
    Defend(i64),
    Heal(i64),
    None,
}
