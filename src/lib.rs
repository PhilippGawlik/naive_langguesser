extern crate clap;
#[macro_use]
extern crate lazy_static; //compile regex only once in loops
extern crate itertools;
extern crate regex;

use config::Sigma;
use errors::GuessingError;
use errors::ModellingError;
use models::Inferer;
use models::LanguageModel;
use models::TextModel;
use std::fs;
use std::path::Path;
use utils::sort_by_second_element;

// these mod statements are just for the compiler to include the modules
mod utils;
// public so main.rs can use the config
pub mod config;
pub mod errors;
pub mod models;
pub mod ngram;
pub mod smoothing;
pub mod text_processing;

pub fn model(config: config::ModelConfig) -> Result<(), ModellingError> {
    let sigma = match config.sigma {
        Sigma::Test => {
            "abcdefghijklmnopqrstuvwxyz0123456789".to_string()
        }
    };
    let raw_text: String = fs::read_to_string(&config.filename)
        .expect(&format!("Failed to read language example from file: {}", &config.filename));
    let _text_model = TextModel::from_raw_str(
        &sigma[..],
        &raw_text[..],
        Some(config.feature_length))
            .expect(&format!(
                "Failed to build text model from content of file: {}",
                &config.filename));
    let language_model = LanguageModel::from_str(
        &config.modelname,
        &raw_text[..],
        config.sigma,
        config.feature_length)
            .expect(&format!(
                "Failed to build language model from content of file: {}",
                &config.filename));
    // write language model to file
    language_model
        .write_probabilities_to_file(&config.outpath)
        .expect(&format!(
            "Failed to write to {}",
            &config.outpath // config ensures value
        ));
    Ok(())
}

pub fn guess(config: config::GuessConfig) -> Result<(), GuessingError> {
    // get language example
    let unclassified = fs::read_to_string(&config.filename)?
        .replace("\n", "")
        .replace("\t", "");
    // initialise language infering struct
    let path = Path::new("./data/models/");
    let inferer = Inferer::from_models_path(&path)?;
    // infer
    let mut prob_table = inferer.infer(unclassified)?;
    prob_table = sort_by_second_element(prob_table)?;
    for (name, prob) in prob_table {
        println!("Guessing {} with : {}", name, prob);
    }
    Ok(())
}
