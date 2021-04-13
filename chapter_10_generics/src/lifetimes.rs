pub fn run() {
    run_part();
}

fn run_longest_slices() {
    let string = String::from("This is the longest string of all strings");

    let x = &string[0..5];
    let y = &string[2..9];

    let result = longest(x, y);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if (x.len() > y.len()) {
        x
    } else {
        y
    }
}

fn return_string() -> String {
    String::from("fdfdf")
}

struct Part<'a> {
    text: &'a str,
}

fn run_part() {
    let x = String::from("this is an important text");

    let part = Part { text: x.as_str() };

    println!("x is: {}", x);
    println!("text part is: {}", part.text);

    let static_message: &'static str = "I live forever";
}
