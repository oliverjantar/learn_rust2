/*

    Each value in Rust has a variable that’s called its owner.
    There can only be one owner at a time.
    When the owner goes out of scope, the value will be dropped.

*/

pub fn run() {
    fn_one();
    fn_two();
}

fn fn_one() {
    let mut s = String::new();

    s.push_str("Hello");

    println!("{}", s);

    let s2 = s;

    //s doesn't work here anymore
    // println!("{}", s); // would panic

    println!("{}", s2);

    //makes copy on heap
    let s3 = s2.clone();

    println!("{} {}", s2, s3);
}

fn fn_two() {
    let mut s = String::from("This is a string variable");

    s = takes_and_gives_ownership(s);
    println!("{}", s);

    takes_ownership(s);

    //cannot use s after this
    // println!("{}", s);

    let x = gives_ownership();
    println!("{}", x);

    //doesn't give ownership
    let mut x = String::from("this is a string");
    let length = calculate_length(&mut x);
    println!("string \"{}\" has length {}", x, length);

    //calculate length again
    let mut x = String::from("Fresh apple");
    let length = add_string_and_calculate_length2(&x);
    println!("string \"{}\" has length {}", x, length);
}

fn takes_ownership(some_string: String) {
    println!("Taking ownership of {}", some_string);
}

fn takes_and_gives_ownership(mut some_string: String) -> String {
    some_string.push_str(" extra value");
    some_string
}

fn gives_ownership() -> String {
    let s = String::from("new string");
    s
}

fn calculate_length(some_string: &mut String) -> usize {
    some_string.push_str("added string");
    some_string.len()
}

fn add_string_and_calculate_length2(mut slice: &str) -> usize {
    slice = "new";
    slice.len()
}
