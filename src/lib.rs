pub mod io;
pub mod pattern;

use io::Config;
use std::error::Error;

pub fn run(conf: Config) -> Result<(), Box<Error>> {
    let content = conf.load()?;
    println!("With text:\n{}", content);
    Ok(())
}
