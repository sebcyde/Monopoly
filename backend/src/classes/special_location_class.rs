pub mod special_location_class {
    use crate::classes::player_class::player_class::Player;

    pub struct SpecialLocation {
        pub name: String,
        pub board_index: u32,
        pub effect: fn(&mut Player),
    }

    impl SpecialLocation {
        pub fn create_special_location(
            name: String,
            board_index: u32,
            effect: fn(&mut Player),
        ) -> Self {
            SpecialLocation {
                name,
                board_index,
                effect,
            }
        }

        pub fn apply_effect(&self, player: &mut Player) {
            &(self.effect)(player);
        }
    }
}
