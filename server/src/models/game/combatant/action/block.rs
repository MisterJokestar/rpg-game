use crate::models::game::combatant::Health;

pub fn basic_block(health: &mut Health, def: i64) {
    health.defend(def);
}
