use crate::models::game::combatant::Health;

pub fn basic_attack(target: &mut Health, dmg: i64) {
    target.hurt(dmg);
}
