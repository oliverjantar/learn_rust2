mod generic_types;
mod lifetimes;
mod traits;

fn main() {
    generic_types::run();
    traits::run();
    lifetimes::run();

    // beginning();
}

fn beginning() {
    let number_list = vec![54, 2344, 66, 77, 34545, 235621, 5, 22];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("{}", largest);

    let second_largest = largest_number(&number_list);
    println!("{}", second_largest);
}

fn largest_number(number_list: &Vec<i32>) -> &i32 {
    let mut largest_number = &number_list[0];

    for number in number_list {
        if number > largest_number {
            largest_number = number;
        }
    }

    largest_number
}

//this is a better implementation as it takes any concrete slice of i32 values
fn largest(number_list: &[i32]) -> &i32 {
    let mut largest_number = &number_list[0];

    for number in number_list {
        if number > largest_number {
            largest_number = number;
        }
    }

    largest_number
}
