use std::env;
use std::env::Args;

pub struct Config {
    pub query: String,
    pub case_sensitive: bool,
    pub filename: String
}

impl Config {
    pub fn new(args: Args) -> Result<Config, &'static str> {
        let mut args = args.skip(1);

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("You forgot the query")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("You left out the filename")
        };

        let case_sensitive = env::var("IGNORE_CASE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}
