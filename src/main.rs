extern crate naive_langguesser;
#[macro_use]
extern crate clap;

use std::process;
use clap::App;
use naive_langguesser::Config;

// Todo
// 1. introduce yaml to vim

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let config = Config::new(&matches).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = naive_langguesser::model(&config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}
