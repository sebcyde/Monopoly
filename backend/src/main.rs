mod game_events;
pub mod helper_functions;
mod types;

fn main() {
    use crate::game_events::game_events::*;
    use crate::helper_functions::helpers::*;
    use crate::types::Types::*;

    println!("Starting Main Monopoly Fn");

    // Initialising stuff

    let mut player_names: Vec<String> = Vec::new();
    loop {
        let input: String = get_input("Please enter player name...");

        if input.is_empty() {
            break;
        } else {
            player_names.push(input)
        }
    }

    let mut community_chest_cards: Vec<EventCard> = Vec::new();
    let mut property_cards: Vec<PropertyCard> = Vec::new();
    let mut chance_cards: Vec<EventCard> = Vec::new();
    let mut board: Vec<Box<dyn Card>> = Vec::new();
    let mut players: Vec<Player> = Vec::new();

    // Combine all the card vectors into the board vector

    for property_card in property_cards {
        board.push(Box::new(property_card) as Box<dyn Card>);
    }

    for chance_card in chance_cards {
        board.push(Box::new(chance_card) as Box<dyn Card>);
    }

    for community_chest_card in community_chest_cards {
        board.push(Box::new(community_chest_card) as Box<dyn Card>);
    }

    create_players(&mut players, player_names, "normal");
    create_community_chest_cards(&mut community_chest_cards);
    create_chance_cards(&mut chance_cards);

    fn check(
        players: Vec<Player>,
        community_chest_cards: Vec<EventCard>,
        chance_cards: Vec<EventCard>,
    ) {
        println!(" ");
        println!("Players:");
        println!(" ");
        for i in 0..players.len() {
            println!("{}", players[i].name);
            println!("{}", players[i].cash_balance);

            println!(" ")
        }
        println!(" ");
        println!("Community Chests");
        println!(" ");
        for i in 0..community_chest_cards.len() {
            println!("{}", community_chest_cards[i].name);
            println!("{}", community_chest_cards[i].description);
            println!(" ")
        }
        println!(" ");
        println!("Chances");
        println!(" ");
        for i in 0..chance_cards.len() {
            println!("{}", chance_cards[i].name);
            println!("{}", chance_cards[i].description);
            println!(" ")
        }
        println!(" ");
    }
    check(players, community_chest_cards, chance_cards);

    // Begin Game HERE
    println!("Starting game event loop.");
    players[0].current_player = true;

    let mut bankrupt_players: Vec<Player> = Vec::new();

    while players.len() > 1 {
        for player in players {
            let dice_roll: u32 = roll_dice();
            calculate_player_position(
                player.current_position,
                dice_roll,
                board.len().try_into().unwrap(),
            );
        }
    }

    println!("End of Main Monopoly Fn");
}
