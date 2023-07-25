pub mod Types {

    use std::option::Option;

    pub trait Card {
        fn activate_effect(&mut self, players: &mut Vec<&mut Player>);
        fn check_ownership(&self, current_player: &Player, players: &mut Vec<Player>);
    }

    impl Card for EventCard {
        fn activate_effect(&mut self, players: &mut Vec<&mut Player>) {
            (self.effect)(players);
        }

        fn check_ownership(&self, current_player: &Player, players: &mut Vec<Player>) {}
    }

    impl Card for PropertyCard {
        fn activate_effect(&mut self, players: &mut Vec<&mut Player>) {}

        fn check_ownership(&self, current_player: &Player, players: &mut Vec<Player>) {
            if let Some(owner) = &self.owner {
                if owner.name != current_player.name {
                    match (self.houses_amount, self.hotel_amount) {
                        (Some(1), _) => println!("Rent: {}", self.one_house_rent),
                        (Some(2), _) => println!("Rent: {}", self.two_house_rent),
                        (Some(3), _) => println!("Rent: {}", self.three_house_rent),
                        (Some(_), Some(_)) => println!("Rent: {}", self.hotel_rent),
                        _ => println!("Rent: {}", self.rent),
                    }
                } else {
                    println!("You own this property.");
                }
            } else {
                println!("Property is not owned.");
            }
        }
    }

    pub struct EventCard {
        pub event: String,
        pub name: String,
        pub description: String,
        pub effect: Box<dyn FnMut(&mut Vec<&mut Player>)>,
    }

    pub struct PropertyCard {
        pub name: String,
        pub price: u32,
        pub mortgage_value: u32,
        pub rent: u32,
        pub one_house_rent: u32,
        pub two_house_rent: u32,
        pub three_house_rent: u32,
        pub hotel_rent: u32,
        pub owner: Option<Player>,
        pub houses_amount: Option<u32>,
        pub hotel_amount: Option<u32>,
    }

    pub struct Player {
        pub name: String,
        pub cash_balance: u32,
        pub owned_property: Option<Vec<PropertyCard>>,
        pub owned_event_cards: Option<Vec<EventCard>>,
        pub current_position: u32,
        pub current_player: bool,
        pub active: bool,
    }

    pub struct Location {
        pub name: String,
        pub relevant_property: Option<PropertyCard>,
        pub effect: Box<dyn FnMut(&mut Vec<&mut Player>)>,
    }
}
