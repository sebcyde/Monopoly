pub mod property_cards {
    use crate::types::Types::{EventCard, Player, PropertyCard};

    pub fn create_property_cards() -> Vec<PropertyCard> {
        let brown_one: PropertyCard = PropertyCard {
            name: String::from("Old Kent Road"),
            price: 120,
            mortgage_value: 60,     // Example value for mortgage value
            one_house_rent: 40,     // Example value for rent with one house
            two_house_rent: 60,     // Example value for rent with two houses
            three_house_rent: 80,   // Example value for rent with three houses
            hotel_rent: 100,        // Example value for rent with a hotel
            owner: None,            // Assuming the property is unowned at the beginning of the game
            houses_amount: Some(0), // Assuming no houses are built on the property at the beginning
            hotel_amount: Some(0),  // Assuming no hotels are built on the property at the beginning
        };

        let brown_two: PropertyCard = PropertyCard {
            name: String::from("Whitechapel Road"),
            price: 60,
            mortgage_value: 30,     // Example value for mortgage value
            one_house_rent: 20,     // Example value for rent with one house
            two_house_rent: 40,     // Example value for rent with two houses
            three_house_rent: 60,   // Example value for rent with three houses
            hotel_rent: 80,         // Example value for rent with a hotel
            owner: None,            // Assuming the property is unowned at the beginning of the game
            houses_amount: Some(0), // Assuming no houses are built on the property at the beginning
            hotel_amount: Some(0),  // Assuming no hotels are built on the property at the beginning
        };

        let station_one: PropertyCard = PropertyCard {
            name: String::from("Kings Cross"),
            price: 120,
            mortgage_value: todo!(),
            one_house_rent: todo!(),
            two_house_rent: todo!(),
            three_house_rent: todo!(),
            hotel_rent: todo!(),
            owner: todo!(),
            houses_amount: todo!(),
            hotel_amount: todo!(),
        };

        let light_blue_one: PropertyCard = PropertyCard {
            name: String::from("Angel, Islington"),
            price: 120,
            mortgage_value: todo!(),
            one_house_rent: todo!(),
            two_house_rent: todo!(),
            three_house_rent: todo!(),
            hotel_rent: todo!(),
            owner: todo!(),
            houses_amount: todo!(),
            hotel_amount: todo!(),
        };

        let light_blue_two: PropertyCard = PropertyCard {
            name: String::from("Euston"),
            price: 120,
            mortgage_value: todo!(),
            one_house_rent: todo!(),
            two_house_rent: todo!(),
            three_house_rent: todo!(),
            hotel_rent: todo!(),
            owner: todo!(),
            houses_amount: todo!(),
            hotel_amount: todo!(),
        };

        let light_blue_three: PropertyCard = PropertyCard {
            name: String::from("Pentonville Road"),
            price: 120,
            mortgage_value: todo!(),
            one_house_rent: todo!(),
            two_house_rent: todo!(),
            three_house_rent: todo!(),
            hotel_rent: todo!(),
            owner: todo!(),
            houses_amount: todo!(),
            hotel_amount: todo!(),
        };
    }
}
