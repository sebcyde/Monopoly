pub mod transfer_property {
    use crate::classes::{
        player_class::player_class::Player, property_class::property_class::PropertyCard,
    };

    pub fn transfer_property(property: &mut PropertyCard, new_owner: Player) {
        property.update_property_owner(new_owner);
        // new_owner.owned_property.unwrap().push(property.clone());
    }
}
