extern crate naive_langguesser;
#[macro_use] // necessary for yaml config of clap
extern crate clap;

use clap::App;
use naive_langguesser::config::Config;
use naive_langguesser::config::Mode;
use std::process;

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let config = Config::new(matches);
    match config.application_mode {
        Mode::Model => process::exit(match naive_langguesser::model(config) {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("Application error: {:?}", err);
                1
            }
        }),
        Mode::Guess => process::exit(match naive_langguesser::guess(config) {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("Application error: {:?}", err);
                1
            }
        }),
    };
}
