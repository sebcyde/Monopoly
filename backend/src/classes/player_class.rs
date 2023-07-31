pub mod player_class {
    use crate::classes::{
        event_card_class::event_card_class::EventCard, property_class::property_class::PropertyCard,
    };

    pub struct Player {
        pub name: String,
        pub cash_balance: u32,
        pub owned_property: Option<Vec<PropertyCard>>,
        pub owned_event_cards: Option<Vec<EventCard>>,
        pub current_position: u32,
        pub current_player: bool,
        pub active: bool,
    }

    impl Player {
        pub fn create_player(
            name: String,
            cash_balance: u32,
            owned_property: Option<Vec<PropertyCard>>,
            owned_event_cards: Option<Vec<EventCard>>,
            current_position: u32,
            current_player: bool,
            active: bool,
        ) -> Self {
            Player {
                name,
                cash_balance,
                owned_property,
                owned_event_cards,
                current_position,
                current_player,
                active,
            }
        }

        pub fn update_board_position(&mut self, new_position: u32) {
            self.current_position = new_position;
        }

        pub fn increase_player_cash(&mut self, amount: u32) {
            self.cash_balance = self.cash_balance + amount;
        }

        pub fn decrease_player_cash(&mut self, amount: u32) {
            self.cash_balance = self.cash_balance - amount;
        }
    }
}
