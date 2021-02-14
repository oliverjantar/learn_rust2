use std::collections::HashMap;

pub fn run() {
    create_new();
    zip();
    owned_values();
}

fn create_new() {
    let mut scores = HashMap::new();

    scores.insert(String::from("red"), 10);
    scores.insert(String::from("green"), 49);

    println!("{:?}", scores);
}

fn zip() {
    let mut teams = vec!["blue", "black"];

    let points = vec![4, 6];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(points.into_iter()).collect();

    println!("{:?}", scores);

    let team = String::from("blue");
    let score = scores.get(&team[..]);

    if let Some(s) = score {
        println!("score of {} is {}", team, s);
    }

    for (key, value) in &scores {
        println!("key: {}, value: {}", key, value)
    }
}

fn owned_values() {
    let integer = 5;

    let string = String::from("some value");

    let mut map = HashMap::new();
    map.insert(integer, string);

    println!("{}", integer);
    // println!("{}", string); // cannot use as it was moved
}
