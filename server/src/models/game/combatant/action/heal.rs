use crate::models::game::combatant::Health;

pub fn basic_heal(target: &mut Health, heal: i64) {
    target.heal(heal);
}
