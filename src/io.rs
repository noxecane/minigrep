use std::env;

pub struct Config {
    pub query: String,
    pub case_sensitive: bool,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Usage: minigrep <query> <filename>");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("IGNORE_CASE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}
