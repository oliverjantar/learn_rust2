pub fn run() {
    let number_list = vec![45, 7, 33, 2323, 667, 88];

    let char_list = vec!['f', 'd', 'j', 'v'];

    let largest_number = largest_i32(&number_list);
    println!("{}", largest_number);

    let lrg_char = largest_char(&char_list);
    println!("{}", lrg_char);
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
