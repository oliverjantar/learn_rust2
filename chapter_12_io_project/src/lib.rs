mod config;
use std::error::Error;
use std::fs;

pub use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("{}", contents);

    Ok(())
}
