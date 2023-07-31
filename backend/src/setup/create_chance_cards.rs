pub mod chance_cards {
    use crate::classes::{
        event_card_class::event_card_class::{Effect, EventCard},
        player_class::player_class::Player,
    };

    pub fn create_chance_cards(chance_cards: &mut Vec<EventCard>) {
        let c1: EventCard = EventCard {
            name: String::from("Advance to Go"),
            description: String::from("Advance token to Go (Collect $200)"),
            effect: Effect::SinglePlayer(Box::new(|player: &mut Player| {
                player.update_board_position(0);
                player.increase_player_cash(200);
                println!("{}'s position is now GO. Awarded 200 cash.", player.name);
            })),
        };

        let c2: EventCard = EventCard {
            name: String::from("Go to jail"),
            description: String::from("Go directly to jail."),
            effect: Effect::SinglePlayer(Box::new(|player: &mut Player| {
                player.update_board_position(20);
                println!("{}'s position is now JAIL.", player.name);
            })),
        };

        let c3: EventCard = EventCard {
            name: String::from("Pay 50"),
            description: String::from("you lost your wallet. Lose 50"),
            effect: Effect::SinglePlayer(Box::new(|player: &mut Player| {
                player.decrease_player_cash(50);
                println!("{} lost 50.", player.name);
            })),
        };

        let c4: EventCard = EventCard {
            name: String::from("gain 20"),
            description: String::from("You won a scratchcard. Gain 20"),
            effect: Effect::SinglePlayer(Box::new(|player: &mut Player| {
                player.increase_player_cash(20);
                println!("{} gained 20.", player.name);
            })),
        };

        // effect: Effect::MultiPlayer(Box::new(|players: &mut Vec<&mut Player>| {
        //     players[0].current_position = 20;
        //     println!("Player position is now JAIL.");
        // })),

        // Loop add
        chance_cards.push(c1);
        chance_cards.push(c2);
        chance_cards.push(c3);
        chance_cards.push(c4);
    }
}
