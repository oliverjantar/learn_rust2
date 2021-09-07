pub fn run() {
    let boundary = 1000;
    let mut array = Vec::new();

    array = multiples(3, boundary, array);
    println!("{:?}", array);
    array = multiples(5, boundary, array);
    println!("{:?}", array);

    let mut sum = 0;
    for item in array {
        sum = sum + item;
    }
    println!("{}", sum);
}

fn multiples(arg: i32, boundary: i32, mut multiples: Vec<i32>) -> Vec<i32> {
    let mut multiple = 1;
    while multiple * arg < boundary {
        if !multiples.contains(&(multiple * arg)) {
            multiples.push(multiple * arg);
        }
        multiple += 1;
    }
    multiples
}
