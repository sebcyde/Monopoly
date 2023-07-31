pub mod roll_dice {
    use rand::Rng;

    pub fn roll_dice() -> u32 {
        let mut rng = rand::thread_rng();
        return rng.gen_range(1..=12);
    }
}
