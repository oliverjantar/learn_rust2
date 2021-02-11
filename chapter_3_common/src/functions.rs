pub fn run() {
    function_one(5);
    let x = another_fun(6);
    println!("{}", x);
}

fn function_one(x: u32) {
    let b = x * 4;
    println!("b is {}", b);
}

fn another_fun(x: u32) -> u32 {
    let y = {
        println!("here x is {}", x);
        let x = 3;
        x + 1
    };
    println!("here y is {}", y);
    println!("here x is {}", x);

    x
}
