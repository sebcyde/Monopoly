pub mod classes {
    pub mod event_card_class;
    pub mod player_class;
    pub mod property_class;
    pub mod special_location_class;
    pub mod station_class;
    pub mod utility_class;
}

pub mod setup {
    pub mod create_chance_cards;
    pub mod create_community_chest_cards;
    pub mod create_player;
    pub mod create_property_cards;
}

pub mod game_events {
    pub mod bankrupt_player;
    pub mod roll_dice;
    pub mod transfer_cash;
    pub mod transfer_property;
}

pub mod helpers {
    pub mod debug_functions;
    pub mod get_user_input;
    pub mod set_player_position;
}

use game_events::{
    roll_dice::roll_dice::roll_dice, transfer_cash::transfer_cash::*,
    transfer_property::transfer_property::transfer_property,
};

use helpers::{debug_functions::debug::*, get_user_input::get_user_input::get_user_input};

use setup::{
    create_chance_cards::chance_cards::create_chance_cards,
    create_community_chest_cards::community_chest_cards::create_community_chest_cards,
    create_player::create_player::create_players,
};

use crate::{
    classes::{
        event_card_class::event_card_class::EventCard, player_class::player_class::Player,
        property_class::property_class::PropertyCard,
        special_location_class::special_location_class::SpecialLocation,
    },
    game_events::bankrupt_player::bankrupt_player::bankrupt_player,
    helpers::set_player_position::set_player_position::set_player_position,
};

fn main() {
    enum Cards {
        PropertyCard(PropertyCard),
        EventCard(EventCard),
        SpecialLocation(SpecialLocation),
    }

    println!("Starting Main Monopoly Fn");

    // Initialising stuff
    let mut community_chest_cards: Vec<EventCard> = Vec::new();
    let mut property_cards: Vec<PropertyCard> = Vec::new();
    let mut bankrupt_players: Vec<&mut Player> = Vec::new();
    let mut chance_cards: Vec<EventCard> = Vec::new();
    let mut player_names: Vec<String> = Vec::new();
    let mut players: Vec<&mut Player> = Vec::new();
    let mut current_player_index: usize = 0;
    let mut board: Vec<Cards> = Vec::new();
    let mut turn: u32 = 0;

    create_players(&mut players, &mut player_names, "normal");
    create_community_chest_cards(&mut community_chest_cards);
    create_chance_cards(&mut chance_cards);

    // debug
    check_players(&players);
    check_chance_cards(&chance_cards);
    check_community_chest_cards(&community_chest_cards);

    for property_card in property_cards {
        board.push(Cards::PropertyCard(property_card));
    }
    for chance_card in chance_cards {
        board.push(Cards::EventCard(chance_card));
    }
    for community_chest_card in community_chest_cards {
        board.push(Cards::EventCard(community_chest_card));
    }

    // Begin Game HERE
    println!("Starting game event loop.");
    println!("Board size: {}", board.len());
    println!(" ");
    println!(" ");

    while players.len() > 1 {
        let mut current_player = &mut players[current_player_index];
        println!("Starting {}'s turn.", current_player.name);

        // move player forward
        let dice_roll: u32 = roll_dice();
        set_player_position(
            &mut current_player,
            dice_roll,
            board.len().try_into().unwrap(),
        );

        // pattern match new location & invoke new location effects
        let current_location = board.get(current_player.current_position as usize);

        match current_location {
            Some(Cards::PropertyCard(property_card)) => {
                println!("Current location: {}", property_card.name);
                println!(
                    "Property owner: {}",
                    property_card.owner.as_ref().unwrap().name
                );
            }
            Some(Cards::EventCard(event_card)) => {
                println!("Current location: {}", event_card.name);
                event_card.apply_effect(&mut current_player, &mut players)
            }
            Some(Cards::SpecialLocation(special_location)) => {
                println!("Current location: {}", special_location.name);
                special_location.apply_effect(&mut current_player);
            }
            None => println!(
                "Invalid card at the location: {}",
                current_player.current_position as usize
            ),
        }

        println!(" ");

        if current_player.cash_balance <= 0 {
            bankrupt_player(&mut players, current_player_index);
            println!("{} is bankrupt!", current_player.name)
        }

        println!(" ");
        println!(" ");

        current_player_index = (current_player_index + 1) % players.len();
        turn += 1;
        if turn == 10 {
            break;
        }
    }

    println!("End of Main Monopoly Fn");
}
