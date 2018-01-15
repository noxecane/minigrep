extern crate minigrep;

use std::env;
use std::process;

use minigrep::io::Config;

fn main() {
    let conf = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match minigrep::run(conf) {
        Ok(results) => {
            if results.is_empty() {
                process::exit(1);
            } else {
                for line in results {
                    println!("{}", line);
                }
            }
        },
        Err(e) => {
            eprintln!("Application Error: {}", e);
            process::exit(1);
        }
    }
}
