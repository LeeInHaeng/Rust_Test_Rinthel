extern crate greprs;

use std::env;
use std::process;

use greprs::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = greprs::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

