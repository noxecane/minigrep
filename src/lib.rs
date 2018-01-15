use std::error::Error;
use std::fs::File;
use std::io::prelude::Read;
use io::Config;

pub mod io;
pub mod pattern;

pub fn run<'a>(conf: Config) -> Result<Vec<String>, Box<Error>> {
    let mut f = File::open(conf.filename)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let results = if conf.case_sensitive {
        pattern::search(&conf.query, &content)
    } else {
        pattern::insensitive_search(&conf.query, &content)
    };

    Ok(results.into_iter().map(String::from).collect())
}
