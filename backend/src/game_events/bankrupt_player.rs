pub mod bankrupt_player {
    use crate::classes::player_class::player_class::Player;

    pub fn bankrupt_player(
        active_players: &mut Vec<&mut Player>,
        // bankrupt_players: &mut Vec<&mut Player>,
        current_player_index: usize,
    ) {
        if current_player_index < active_players.len() {
            let removed_player: &mut Player = active_players.remove(current_player_index);
            // bankrupt_players.push(removed_player);
        }
    }
}
