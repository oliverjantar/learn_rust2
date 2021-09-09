pub fn run() {
    simple_init();
}

fn simple_init() {
    let b = Box::new(5);
    println!("b is {}", b);
}
