use std::env;
use std::process;

use chapter_12_io_project::Config;

// cargo run banish C:/Users/admin/Desktop/temp/poem.txt
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //better than unwrap_or_else as we don't use the return value
    if let Err(e) = chapter_12_io_project::run(config) {
        println!("App error: {}", e);
        process::exit(1);
    }
}
