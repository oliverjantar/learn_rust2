pub fn run() {
    let mut user = Person {
        age: 1,
        is_married: false,
        name: String::from("Test"),
    };

    user.age = 5;

    println!("{:?}", user);

    let user = Person::new(String::from("Tim"), 45, false);

    println!("{:?}", user);

    println!("user is married? {}", user.get_is_married());

    struct Color(i32, i32, i32); //tuple struct

    let x = Color(5, 45, 22);

    struct UnitLikeStruct();

    let y = UnitLikeStruct();
}

#[derive(Debug)] //can now use println!("{:?}",Person)
struct Person {
    name: String,
    age: u8,
    is_married: bool,
}

impl Person {
    fn new(name: String, age: u8, is_married: bool) -> Person {
        Person {
            age,
            name,
            is_married,
        }
    }

    fn get_is_married(&self) -> bool {
        self.is_married
    }
}
