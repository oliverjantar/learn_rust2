use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

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
        let x = [String::from("test")];
        let result = Config::new(&x).unwrap_err();
        assert_eq!(result, "Not enough arguments!");
    }

    #[test]
    fn creates_new_config() {
        let x = [
            String::from("not used param"),
            String::from("query"),
            String::from("filename"),
        ];
        let result = Config::new(&x).unwrap();

        assert_eq!(result.filename, x[2]);
        assert_eq!(result.query, x[1]);
    }
}
