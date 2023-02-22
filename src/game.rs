use rand::Rng;

pub(crate) struct Game {
    pub number: u32,
    pub number_of_tries: u16,
}

impl Game {
    pub fn increment_tries(&mut self) {
        self.number_of_tries += 1;
    }
}

impl Default for Game {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let number: u32 = rng.gen_range(0..=1_000_000);

        Game {
            number,
            number_of_tries: 0,
        }
    }
}