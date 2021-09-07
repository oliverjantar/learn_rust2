use core::panic;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); //name of a program is always the first parameter

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get query parameter"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get filename parameter"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn config_new_returns_err() {
        let x = vec![String::from("test")];
        let result = Config::new(x.into_iter()).unwrap_err();
        assert_eq!(result, "Didn't get query parameter");

        let x = vec![String::from("test"), String::from("query")];
        let result = Config::new(x.into_iter()).unwrap_err();
        assert_eq!(result, "Didn't get filename parameter");
    }

    #[test]
    fn creates_new_config() {
        let x = vec![
            String::from("not used param"),
            String::from("query"),
            String::from("filename"),
        ];
        let result = Config::new(x.into_iter()).unwrap();

        assert_eq!(result.query, String::from("query"));
        assert_eq!(result.filename, String::from("filename"));
    }
}
