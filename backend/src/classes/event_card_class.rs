pub mod event_card_class {
    use crate::classes::player_class::player_class::Player;

    pub struct EventCard {
        pub name: String,
        pub description: String,
        pub effect: Effect,
    }

    pub enum Effect {
        SinglePlayer(Box<dyn Fn(&mut Player)>),
        MultiPlayer(Box<dyn Fn(&mut Vec<&mut Player>)>),
    }

    impl EventCard {
        pub fn create_event_card(name: String, description: String, effect: Effect) -> Self {
            EventCard {
                name,
                description,
                effect,
            }
        }

        pub fn apply_effect(&self, player: &mut Player, players: &mut Vec<&mut Player>) {
            match &self.effect {
                Effect::SinglePlayer(effect) => effect(player),
                Effect::MultiPlayer(effect) => effect(players),
            }
        }
    }
}
