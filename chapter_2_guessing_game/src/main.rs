use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game");

    println!("guess number from 1 to 100");

    let random_int: u32 = rand::thread_rng().gen_range(1, 101);

    dbg!(random_int);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You haven't entered a number!");
                continue;
            }
        };

        match guess.cmp(&random_int) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
