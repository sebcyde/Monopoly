pub mod transfer_cash {
    use crate::classes::player_class::player_class::Player;

    pub fn transfer_from_bank(recieving_user: &mut Player, amount: u32) {
        recieving_user.increase_player_cash(amount);
    }

    pub fn transfer_to_bank(sending_user: &mut Player, amount: u32) {
        sending_user.decrease_player_cash(amount);
    }

    pub fn transfer_to_player(recieving_user: &mut Player, sending_user: &mut Player, amount: u32) {
        sending_user.decrease_player_cash(amount);
        recieving_user.increase_player_cash(amount);
    }
}
