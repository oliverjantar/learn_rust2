#[derive(PartialEq, Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 5,
        };
        let smaller = Rectangle {
            width: 4,
            height: 3,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 5,
        };
        let smaller = Rectangle {
            width: 4,
            height: 3,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
