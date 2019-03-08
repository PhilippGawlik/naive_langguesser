extern crate clap;
#[macro_use]
extern crate lazy_static; //compile regex only once in loops
extern crate itertools;
extern crate regex;

use errors::GuessingError;
use errors::ModellingError;
use models::Inferer;
use models::LanguageModel;
use std::fs;
use std::path::Path;
use utils::sort_by_probability;

// these mod statements are just for the compiler to include the modules
mod utils;
// public so main.rs can use the config
pub mod config;
pub mod errors;
pub mod models;
pub mod ngram;

pub fn model(config: config::Config) -> Result<(), ModellingError> {
    // get language example
    let content = fs::read_to_string(&config.filename)?
        .replace("\n", "")
        .replace("\t", "");
    // build language model
    let language_model = LanguageModel::from_str(
        config.modelname.as_ref().unwrap(), // config ensures existence of value
        &content[..],
    )?;
    // write language model
    language_model
        .write_probabilities_to_file(
            &config
            .outpath
            .as_ref()
            .unwrap()  // config ensures value
            [..],
        )
        .expect(&format!(
            "Failed to write to {}",
            &config.outpath.as_ref().unwrap() // config ensures value
        ));
    Ok(())
}

pub fn guess(config: config::Config) -> Result<(), GuessingError> {
    // get language example
    let unclassified = fs::read_to_string(&config.filename)?
        .replace("\n", "")
        .replace("\t", "");
    // initialise language infering struct
    let path = Path::new("./data/models/");
    let inferer = Inferer::from_models_path(&path)?;
    // infer
    let mut prob_table = inferer.infer(unclassified)?;
    prob_table = sort_by_probability(prob_table)?;
    for (name, prob) in prob_table {
        println!("Guessing {} with : {}", name, prob);
    }
    Ok(())
}
