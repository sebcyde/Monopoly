pub mod community_chest_cards {
    use crate::classes::{
        event_card_class::event_card_class::{Effect, EventCard},
        player_class::player_class::Player,
    };

    pub fn create_community_chest_cards(community_chest_cards: &mut Vec<EventCard>) {
        let c1: EventCard = EventCard {
            name: String::from("Gain 20"),
            description: String::from("You won a scratchcard. Gain 20"),
            effect: Effect::SinglePlayer(Box::new(|player: &mut Player| {
                player.increase_player_cash(20);
                println!("{} gained 20.", player.name);
            })),
        };

        let c2: EventCard = EventCard {
            name: String::from("Lose 100"),
            description: String::from("You were robbed!. Lose 100"),
            effect: Effect::SinglePlayer(Box::new(|player: &mut Player| {
                player.decrease_player_cash(100);
                println!("{} was robbed and lost 100!", player.name);
            })),
        };

        // Loop add
        community_chest_cards.push(c1);
        community_chest_cards.push(c2);
    }
}
