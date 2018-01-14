use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }

    pub fn load(&self) -> Result<String, Box<Error>> {
        let mut f = File::open(&self.filename)?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        Ok(content)
    }
}
