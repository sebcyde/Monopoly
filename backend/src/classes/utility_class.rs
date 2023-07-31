pub mod utility_class {
    use crate::classes::player_class::player_class::Player;

    pub struct UtilityCard {
        pub name: String,
        pub price: u32,
        pub mortgage_value: u32,
        pub rent: u32,
        pub owner: Option<Player>,
        pub board_index: u32,
        pub mortgaged: bool,
    }

    impl UtilityCard {
        pub fn create_utility_card(
            name: String,
            price: u32,
            mortgage_value: u32,
            rent: u32,
            owner: Option<Player>,
            board_index: u32,
            mortgaged: bool,
        ) -> Self {
            UtilityCard {
                name,
                price,
                mortgage_value,
                rent,
                owner,
                board_index,
                mortgaged,
            }
        }

        pub fn get_rent_amount(&self, dice_roll: u32) -> u32 {
            return &self.rent * dice_roll;
        }

        pub fn mortgage_property(&mut self) {
            self.mortgaged = !self.mortgaged;
            if self.mortgaged {
                self.owner.as_mut().unwrap().cash_balance += self.mortgage_value;
            } else {
                self.owner.as_mut().unwrap().cash_balance -= self.mortgage_value;
            }
            println!("{} mortage status: {}", self.name, self.mortgaged);
        }

        pub fn get_property_owner(&self) -> Option<&Player> {
            match &self.owner {
                Some(owner) => Some(owner),
                None => None,
            }
        }

        pub fn update_property_owner(&mut self, new_owner: Player) {
            self.owner = Some(new_owner);
            println!(
                "{}'s owner is now: {}",
                &self.name,
                &self.owner.as_mut().unwrap().name
            );
        }
    }
}
