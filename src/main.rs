extern crate naive_langguesser;
#[macro_use] // necessary for yaml config of clap
extern crate clap;

use clap::App;
use naive_langguesser::config::GuessConfig;
use naive_langguesser::config::ModelConfig;
use std::process;

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(matches) = matches.subcommand_matches("model") {
        let config = ModelConfig::new(matches);
        process::exit(match naive_langguesser::model(config) {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("Application error: {:?}", err);
                1
            }
        });
    } else if let Some(matches) = matches.subcommand_matches("guess") {
        let config = GuessConfig::new(matches);
        process::exit(match naive_langguesser::guess(config) {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("Application error: {:?}", err);
                1
            }
        });
    };
}
