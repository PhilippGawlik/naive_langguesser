//extern crate clap;
#[macro_use]
extern crate lazy_static; //compile regex only once in loops
extern crate itertools;
extern crate regex;

use errors::GuessingError;
use errors::ModellingError;
use inferer::Inferer;
use models::count_model::CountModel;
use models::probability_model::ProbabilityModel;
use models::text_model::TextModel;
use std::fs;

pub mod config;
mod errors;
mod inferer;
mod models;
mod ngram;
mod smoothing;
mod text_processing;
mod utils;

/// Definition of execution modes of `naive_langguesser`
///
/// # Model
///
/// Calculate a probability based language model from a text example file.
///
/// # Guess
///
/// Classify a text with the most probable language based on present language models.
pub enum Mode {
    Model,
    Guess,
}

/// Calculate a probability based language model from a text example file
///
/// # Arguments
///
/// * `config` - a struct holding config settings, partly given through cli
pub fn model(config: config::ModelConfig) -> Result<(), ModellingError> {
    let raw_text: String = fs::read_to_string(&config.filename)?;
    let text_model = TextModel::from_raw(
        &raw_text[..],
        config.sigma_id,
        config.set_marker,
        config.ngram_length,
    )?;
    let mut count_model = CountModel::from_sigma(text_model.get_sigma(), config.ngram_length)?;
    let mut probability_model = ProbabilityModel::from_name(&config.modelname)?;
    count_model.count_ngrams_from_text_model(&text_model)?;
    count_model.smooth(&config.smoothing_type)?;
    probability_model.add_unigram_probabilities(&count_model)?;
    probability_model.add_ngram_probabilities(&count_model)?;
    probability_model.write_to_file(&config.outpath)?;
    Ok(())
}

/// Classify a text with the most probable language based on available language models
///
/// # Arguments
///
/// * `config` - a struct holding config settings, partly given through cli
pub fn guess(config: config::GuessConfig) -> Result<(), GuessingError> {
    let raw_unclassified = fs::read_to_string(&config.filename)?;
    let text_model = TextModel::from_raw(
        &raw_unclassified[..],
        config.sigma_id,
        config.set_marker,
        config.ngram_length,
    )?;
    let inferer: Inferer =
        Inferer::from_models_dir(&config.model_dir, config.ngram_length, config.in_parallel)?;
    let prob_table = inferer.infer(&text_model)?;
    for (name, prob) in prob_table {
        println!("Guessing {} with : {}", name, prob);
    }
    Ok(())
}
