mod guess;
mod rectangle;

pub fn add_two(x: i32) -> i32 {
    let x = 2;
    x + 2

}

fn greetings(param: &str) -> String {
    format!("Hello.")
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn exploration() {
        assert_eq!(7, add_two(5));
    }

    #[test]
    fn add_two_doesnt_add_three() {
        assert_ne!(add_two(2), 5);
    }

    #[test]
    fn fail_test() {
        // panic!("this test fails");
    }

    #[test]
    #[ignore]
    fn greetings_contain_name() {
        let result = greetings("Maky");
        assert!(
            result.contains("Maky"),
            "Greeting did not contain name, value was {}",
            result
        );
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Does not equal to 4"))
        }
    }
}
