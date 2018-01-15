use std::error::Error;
use std::fs::File;
use std::io::prelude::Read;
use io::Config;

pub mod io;
pub mod pattern;

pub fn run(conf: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(conf.filename)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let results = if conf.case_sensitive {
        pattern::search(&conf.query, &content)
    } else {
        pattern::insensitive_search(&conf.query, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
