pub mod game_events {
    use crate::types::Types::{EventCard, Player};
    use rand::Rng;

    pub enum TransactionType {
        Add,
        Subtract,
    }

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
            current_position: 0,
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
                players[0].current_position = 0;
                players[0].cash_balance += 200;
                println!("Player position is now GO. Player awarded 200 cash.");
            }),
        };

        let c2: EventCard = EventCard {
            event: String::from("Chance"),
            name: String::from("Go to jail"),
            description: String::from("Go directly to jail."),
            effect: Box::new(|players: &mut Vec<&mut Player>| {
                players[0].current_position = 31;
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
                players[0].current_position = 31;
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

    pub fn roll_dice() -> u32 {
        let mut rng = rand::thread_rng();
        return rng.gen_range(1..=12);
    }

    pub fn calculate_player_position(
        current_player: &mut Player,
        dice_roll: u32,
        board_length: u32,
    ) {
        println!("Dice roll: {}", dice_roll);
        // let mut current_position: &mut u32 = current_player.current_position;
        let new_position = current_player.current_position + dice_roll;

        if new_position >= board_length {
            let remainder: u32 = new_position % board_length;
            current_player.cash_balance += 200;
            current_player.current_position = remainder;
            println!(
                "(> BL) - Player new position: {}",
                current_player.current_position
            );
            println!(
                "(> BL) - Player new balance: {}",
                current_player.cash_balance
            );
        } else {
            current_player.current_position = new_position;
            println!(
                "(< BL) - Player new position: {}",
                current_player.current_position
            );
        }
    }

    pub fn process_transaction(
        current_player: &mut Player,
        amount: u32,
        transaction_type: TransactionType,
    ) {
        match transaction_type {
            TransactionType::Add => {
                current_player.cash_balance += amount;
            }
            TransactionType::Subtract => {
                current_player.cash_balance -= amount;
            }
        }
    }
}
