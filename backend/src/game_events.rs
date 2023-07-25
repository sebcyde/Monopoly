pub mod game_events {
    use crate::types::Types::{EventCard, Player, PropertyCard};
    use rand::Rng;

    pub fn create_players(players: &mut Vec<Player>, player_names: Vec<String>, game_type: &str) {
        if game_type == "normal" {
            for i in 0..player_names.len() {
                let current_player: Player = create_player(player_names[i].clone(), "normal");
                players.push(current_player);
            }
        }
    }

    pub fn create_player(name: String, game_type: &str) -> Player {
        let default: Player = Player {
            name,
            cash_balance: 2000,
            owned_property: Some(Vec::new()),
            owned_event_cards: Some(Vec::new()),
            current_position: String::from("Go"),
            current_player: false,
            active: true,
        };

        return default;
    }

    pub fn create_chance_cards(chance_cards: &mut Vec<EventCard>) {
        let c1: EventCard = EventCard {
            event: String::from("Chance"),
            name: String::from("Advance to Go"),
            description: String::from("Advance token to Go (Collect $200)"),
            effect: Box::new(|players: &mut Vec<&mut Player>| {
                players[0].current_position = String::from("Go");
                players[0].cash_balance += 200;
                println!("Player position is now GO. Player awarded 200 cash.");
            }),
        };

        let c2: EventCard = EventCard {
            event: String::from("Chance"),
            name: String::from("Go to jail"),
            description: String::from("Go directly to jail."),
            effect: Box::new(|players: &mut Vec<&mut Player>| {
                players[0].current_position = String::from("Jail");
                println!("Player position is now JAIL.");
            }),
        };

        // Loop add
        chance_cards.push(c1);
        chance_cards.push(c2);
    }

    pub fn create_community_chest_cards(community_chest_cards: &mut Vec<EventCard>) {
        let c1: EventCard = EventCard {
            event: String::from("Community Chest"),
            name: String::from("Go to jail"),
            description: String::from("Go directly to jail."),
            effect: Box::new(|players: &mut Vec<&mut Player>| {
                players[0].current_position = String::from("Jail");
                println!("Player position is now JAIL.");
            }),
        };

        let c2: EventCard = EventCard {
            event: String::from("Community Chest"),
            name: String::from("Win $25 in a lottery"),
            description: String::from("You win a lottery, collect $25 from the Bank."),
            effect: Box::new(|players: &mut Vec<&mut Player>| {
                players[0].cash_balance += 25;
                println!("Player awarded 25 cash.");
            }),
        };

        // Loop add
        community_chest_cards.push(c1);
        community_chest_cards.push(c2);
    }

    pub fn create_property_cards() -> Vec<PropertyCard> {
        let light_blue_one: PropertyCard = PropertyCard {
            name: String::from("light_blue_one"),
            price: 120,
            mortgage_value: todo!(),
            one_house_rent: todo!(),
            two_house_rent: todo!(),
            three_house_rent: todo!(),
            hotel_rent: todo!(),
            owner: todo!(),
            houses_amount: todo!(),
            hotel_amount: todo!(),
        };
    }

    pub fn roll_dice() -> u32 {
        let mut rng = rand::thread_rng();
        return rng.gen_range(1..=12);
    }

    pub fn calculate_player_position(current_position: u32, dice_roll: u32, board_length: u32) {
        if current_position + dice_roll > board_length {
            let remainder: u32 = board_length % (current_position + dice_roll);
            println!("Dice roll remainder: {}", remainder);
            current_position = remainder;
            println!("(IF) - Player current position: {}", current_position);
        } else {
            current_position += dice_roll;
            println!("(ELSE) - Player current position: {}", current_position);
        }
    }
}
