pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("pick int in range 1-100");
        }
        return Guess { value };
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
