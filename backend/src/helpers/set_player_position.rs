pub mod set_player_position {
    use crate::classes::player_class::player_class::Player;

    pub fn set_player_position(current_player: &mut Player, dice_roll: u32, board_length: u32) {
        let new_position = current_player.current_position + dice_roll;
        current_player.update_board_position(new_position);

        println!("Dice roll: {}", dice_roll);
        println!("Player new position: {}", current_player.current_position);

        if new_position >= board_length {
            current_player.increase_player_cash(200);
            println!("Player new balance: {}", current_player.cash_balance);
        }
    }
}
