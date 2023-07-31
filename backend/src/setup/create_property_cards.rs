pub mod property_cards {
    use crate::classes::property_class::property_class::PropertyCard;
    use crate::classes::station_class::station_class::StationCard;

    pub fn create_station_cards(station_cards: &mut Vec<StationCard>) -> &mut Vec<StationCard> {
        station_cards
    }

    pub fn create_property_cards(property_cards: &mut Vec<PropertyCard>) -> &mut Vec<PropertyCard> {
        let brown_one: PropertyCard = PropertyCard::create_property_card(
            String::from("Old Kent Road"),
            60,
            30,
            2,
            10,
            30,
            90,
            160,
            250,
            None,
            Some(0),
            false,
            1,
            false,
            50,
        );
        property_cards.push(brown_one);

        let brown_two: PropertyCard = PropertyCard::create_property_card(
            String::from("Baltic Ave."),
            60,
            30,
            4,
            20,
            60,
            180,
            320,
            450,
            None,
            Some(0),
            false,
            3,
            false,
            50,
        );
        property_cards.push(brown_two);

        let light_blue_one: PropertyCard = PropertyCard::create_property_card(
            String::from("Angel, Islington"),
            100,
            50,
            6,
            30,
            90,
            270,
            400,
            550,
            None,
            Some(0),
            false,
            6,
            false,
            50,
        );
        property_cards.push(light_blue_one);

        let light_blue_two: PropertyCard = PropertyCard::create_property_card(
            String::from("blue_two"),
            100,
            50,
            6,
            30,
            90,
            270,
            400,
            550,
            None,
            Some(0),
            false,
            8,
            false,
            50,
        );
        property_cards.push(light_blue_two);

        let light_blue_three: PropertyCard = PropertyCard::create_property_card(
            String::from("light_blue_three"),
            120,
            60,
            8,
            40,
            100,
            300,
            450,
            600,
            None,
            Some(0),
            false,
            9,
            false,
            50,
        );
        property_cards.push(light_blue_three);

        return property_cards;
    }
}
