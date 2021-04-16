pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Value should be between 1-100, got {}", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Value should be between 1-100, got 1000")]
    fn should_panic_when_guessed_1000() {
        Guess::new(1000);
    }
}
