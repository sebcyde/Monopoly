pub mod Types {

    use std::option::Option;

    pub trait Card {
        fn get_name(&self) -> &String;
    }
    pub struct EventCard {
        pub event: String,
        pub name: String,
        pub description: String,
        pub effect: Box<dyn FnMut(&mut Vec<&mut Player>)>,
    }

    impl Card for EventCard {
        fn get_name(&self) -> &String {
            &self.name
        }
    }

    pub struct PropertyCard {
        pub name: String,
        pub price: u32,
        pub mortgage_value: u32,
        pub one_house_rent: u32,
        pub two_house_rent: u32,
        pub three_house_rent: u32,
        pub hotel_rent: u32,
        pub owner: Option<Player>,
        pub houses_amount: Option<u32>,
        pub hotel_amount: Option<u32>,
    }

    impl Card for PropertyCard {
        fn get_name(&self) -> &String {
            &self.name
        }
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
