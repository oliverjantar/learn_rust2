mod config;
use std::error::Error;
use std::fs;

pub use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    Ok(())
}

pub fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in text.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let text = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, text));
    }
}
