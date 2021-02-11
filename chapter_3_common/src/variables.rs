const MAX_POINTS: u8 = 100;

pub fn run() {
    let mut x = 5;
    println!("{}", x);

    x = 6;

    println!("{}", x);

    println!("{}", MAX_POINTS);

    shadowing();
}

fn shadowing() {
    let x = 5;

    let x = x * 2;

    println!("{}", x);
}
