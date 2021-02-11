pub fn run() {
    let x = "This is a slice";
    let y = String::from("This is a string");
    let z = &y; // reference to a string

    let word = first_word(x);
    println!("{}", word);

    let word = first_word(&y); //same as next
    println!("{}", word);

    let word = first_word(z);
    println!("{}", word);

    let word = first_word(&y[5..9]);
    println!("word is: {}", &y[5..9]);
    println!("{}", word)
}

fn first_word(some_slice: &str) -> &str {
    let bytes = some_slice.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_slice[0..i];
        }
    }
    return some_slice;
}
