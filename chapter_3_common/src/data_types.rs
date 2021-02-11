pub fn run() {
    tuples();
    arrays();
}

fn tuples() {
    let mut tup: (i32, f64, bool) = (500, 3.2, false);

    println!("{} {} {}", tup.0, tup.1, tup.2);

    tup.2 = true;

    // tup.3 = false; won't build

    //destructuring
    let (x, y, z) = tup;

    println!("{} {} {}", x, y, z);
}

fn arrays() {
    let a = [1, 3, 6, 3, 4];
    println!("{:?}", a);

    let a = [3; 5];
    println!("{:?}", a);

    let first_element = a[0];

    println!("{}", first_element);
}
