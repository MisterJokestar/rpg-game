use crate::models::game::combatant::action::{
    ActionCard, ActionDeck, ActionType, Target
};

pub fn get_fighter_deck() -> ActionDeck {
    let mut total_deck: Vec<ActionCard> = Vec::new();
    total_deck.push(base_attack_card());
    total_deck.push(base_attack_card());
    total_deck.push(base_attack_card());
    total_deck.push(base_attack_card());
    total_deck.push(multi_attack_card());
    total_deck.push(multi_attack_card());
    total_deck.push(multi_defend_card());
    total_deck.push(multi_defend_card());
    total_deck.push(multi_defend_card());
    total_deck.push(multi_defend_card());
    total_deck.push(base_heal_card());
    total_deck.push(base_heal_card());
    ActionDeck::new(total_deck)
}

fn base_attack_card() -> ActionCard {
    ActionCard::create_card(
        "Slice and Dice!".into(),
        Target::Enemy(None),
        ActionType::Attack(8)
    )
}

fn multi_attack_card() -> ActionCard {
    ActionCard::create_card(
        "The Long Slice!".into(),
        Target::AllEnemies,
        ActionType::Attack(6)
    )
}

fn multi_defend_card() -> ActionCard {
    ActionCard::create_card(
        "Hold The Line!".into(),
        Target::AllHeros,
        ActionType::Block(12)
    )
}

fn base_heal_card() -> ActionCard {
    ActionCard::create_card(
        "Stay Strong!".into(),
        Target::HeroSelf,
        ActionType::Heal(8)
    )
}

fn attack_defensivly_card() -> ActionCard {
    let mut card = ActionCard::create_card(
        "Attack Defensivly!".into(),
        Target::Enemy(None),
        ActionType::Attack(6)
    );
    card.add_action(
        Target::HeroSelf,
        ActionType::Block(6)
    );
    card
}

