pub fn run() {
    run_loop();
    run_while();
    loop_collection();
    run_match_if();
}

fn run_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter > 5 {
            break counter * 2;
        }
    };

    println!("result is {}", result);
}

fn run_while() {
    let mut x = 0;

    while x < 3 {
        x += 1;
        println!("{}", x);
    }
}

fn loop_collection() {
    let a = [1, 5, 7, 3, 8, 0, 3];

    for element in a.iter() {
        println!("element is {}", element);
    }

    for element in (1..4).rev() {
        println!("Reverse loop: element is {}", element);
    }
}

fn run_match_if() {
    let x = Some(5);

    match x {
        Some(y) if y < 4 => println!("it is less than 4"),
        _ => println!("it is higher than 4"),
    }
}
