extern crate minigrep;

use std::env;
use std::process;

use minigrep::io::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(conf) {
        println!("Application Error: {}", e);
        process::exit(1);
    };
}
