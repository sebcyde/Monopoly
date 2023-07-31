pub mod debug {
    use crate::classes::{
        event_card_class::event_card_class::EventCard, player_class::player_class::Player,
    };

    pub fn check_players(players: &Vec<&mut Player>) {
        println!(" ");
        println!("Players:");
        println!(" ");
        for i in 0..players.len() {
            println!("{}", players[i].name);
            println!("{}", players[i].cash_balance);

            println!(" ")
        }
        println!(" ");
    }

    pub fn check_community_chest_cards(community_chest_cards: &Vec<EventCard>) {
        println!(" ");
        println!("Community Chests");
        println!(" ");
        for i in 0..community_chest_cards.len() {
            println!("{}", community_chest_cards[i].name);
            println!("{}", community_chest_cards[i].description);
            println!(" ")
        }
        println!(" ");
    }

    pub fn check_chance_cards(chance_cards: &Vec<EventCard>) {
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
}
