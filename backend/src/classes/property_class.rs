pub mod property_class {
    use crate::classes::player_class::player_class::Player;

    pub struct PropertyCard {
        pub name: String,
        pub price: u32,
        pub mortgage_value: u32,
        pub rent: u32,
        pub one_house_rent: u32,
        pub two_house_rent: u32,
        pub three_house_rent: u32,
        pub four_house_rent: u32,
        pub hotel_rent: u32,
        pub owner: Option<Player>,
        pub houses_amount: Option<u32>,
        pub hotel: bool,
        pub board_index: u32,
        pub mortgaged: bool,
        pub build_cost: u32,
    }

    impl PropertyCard {
        pub fn create_property_card(
            name: String,
            price: u32,
            mortgage_value: u32,
            rent: u32,
            one_house_rent: u32,
            two_house_rent: u32,
            three_house_rent: u32,
            four_house_rent: u32,
            hotel_rent: u32,
            owner: Option<Player>,
            houses_amount: Option<u32>,
            hotel: bool,
            board_index: u32,
            mortgaged: bool,
            build_cost: u32,
        ) -> Self {
            PropertyCard {
                name,
                price,
                mortgage_value,
                rent,
                one_house_rent,
                two_house_rent,
                three_house_rent,
                four_house_rent,
                hotel_rent,
                owner,
                houses_amount,
                hotel,
                board_index,
                mortgaged,
                build_cost,
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

        pub fn get_property_owner(&self) -> Option<&Player> {
            match &self.owner {
                Some(owner) => Some(owner),
                None => None,
            }
        }

        pub fn update_property_owner(&mut self, new_owner: Player) {
            if self.hotel || self.houses_amount.unwrap() > 0 {
                println!("Cannot trade a developed property");
                return;
            }

            self.owner = Some(new_owner);
            println!(
                "{}'s owner is now: {}",
                self.name,
                self.owner.as_mut().unwrap().name
            );
        }

        pub fn promote_property(&mut self) {
            if self.houses_amount.unwrap() == 4 {
                self.houses_amount = Some(0);
                self.hotel = true;
                println!("{} now has a hotel.", self.name);
            } else {
                self.houses_amount = Some(self.houses_amount.unwrap() + 1);
                println!(
                    "{} now has {} houses.",
                    self.name,
                    self.houses_amount.unwrap()
                );
            }
        }

        pub fn demote_property(&mut self) {
            if self.houses_amount.unwrap() == 0 {
                println!("{} has no houses.", &self.name);
                return;
            } else if self.hotel {
                self.houses_amount = Some(4);
            } else {
                self.houses_amount = Some(self.houses_amount.unwrap() - 1);
            }
            println!(
                "{} now has {} houses.",
                &self.name,
                self.houses_amount.unwrap()
            );
        }

        pub fn get_rent_amount(self) -> u32 {
            if self.hotel {
                self.hotel_rent;
            };

            match self.houses_amount {
                Some(1) => self.one_house_rent,
                Some(2) => self.two_house_rent,
                Some(3) => self.three_house_rent,
                Some(4) => self.four_house_rent,
                _ => self.rent,
            }
        }
    }
}
