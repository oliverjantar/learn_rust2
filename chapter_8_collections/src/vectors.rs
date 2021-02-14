pub fn run() {
    create_new();
    refs();
    iterate();
    different_types();
}

fn create_new() {
    let mut v: Vec<i32> = Vec::new();

    let v2 = vec![-4, 1, 2, 3];

    v.push(5);
    v.push(23);
}

fn refs() {
    let v: Vec<i32> = vec![3, 5, 62, 32];

    let third = &v[2];

    println!("{}", third);

    let test = &v[1..3];

    match v.get(4) {
        Some(item) => println!("{}", item),
        None => println!("No item"),
    }

    //this returns Option with a reference to an array of i32
    let x = v.get(0..3);
    if let Some(items) = x {
        println!("{:?}", items);
    }
}

fn iterate() {
    let mut v = vec![3, 2, 5, 23];

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }
}

#[derive(Debug)]
enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn different_types() {
    let mut v: Vec<Cell> = vec![Cell::Int(5), Cell::Text(String::from("some text"))];

    for item in &mut v {
        *item = Cell::Float(34.22);
    }

    println!("{:?}", v);
}
