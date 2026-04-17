pub mod user;
pub mod game;

#[derive(Debug, Clone)]
pub enum Action {
    Attack(i32),
    Defend(i32),
    Heal(i32),
    None,
}
