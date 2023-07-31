pub mod station_class {
    use crate::classes::player_class::player_class::Player;

    pub struct StationCard {
        pub name: String,
        pub price: u32,
        pub mortgage_value: u32,
        pub rent: u32,
        pub owner: Option<Player>,
        pub board_index: u32,
        pub mortgaged: bool,
    }

    impl StationCard {
        pub fn create_station_card(
            name: String,
            price: u32,
            mortgage_value: u32,
            rent: u32,
            owner: Option<Player>,
            board_index: u32,
            mortgaged: bool,
        ) -> Self {
            StationCard {
                name,
                price,
                mortgage_value,
                rent,
                owner,
                board_index,
                mortgaged,
            }
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

        pub fn get_rent_amount(&self) -> &u32 {
            return &self.rent;
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
