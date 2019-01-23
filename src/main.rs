extern crate naive_langguesser;

use naive_langguesser::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = naive_langguesser::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}
