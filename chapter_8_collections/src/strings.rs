pub fn run() {
    init();
}

fn init() {
    let data = "rerererdfsf";

    let string_data = data.to_string();

    let mut string_data2 = String::from(data);

    string_data2.push_str("dfdfdf");

    string_data2.push('1');
}
