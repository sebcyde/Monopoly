pub mod create_player {
    use crate::classes::player_class::player_class::Player;
    use crate::helpers::get_user_input::get_user_input::get_user_input;

    pub fn create_players(
        players: &mut Vec<&mut Player>,
        player_names: &mut Vec<String>,
        game_type: &str,
    ) {
        loop {
            let input: String = get_user_input("Please enter player name...");

            if player_names.len() > 3 {
                println!("Max amount of players is 4!");
                break;
            } else if input.is_empty() {
                println!("Starting game.");
                break;
            } else {
                player_names.push(input)
            }
        }

        if game_type == "normal" {
            for i in 0..player_names.len() {
                let mut new_player: Player = Player::create_player(
                    player_names[i].clone(),
                    2000,
                    Some(Vec::new()),
                    Some(Vec::new()),
                    0,
                    false,
                    true,
                );

                players.push(&mut new_player);
            }
        }
    }
}
