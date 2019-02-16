extern crate clap;
#[macro_use] extern crate lazy_static;  //compile regex only once in loops
extern crate regex;

use std::error::Error;
use std::collections::HashMap;
use std::path::Path;
use std::fs;

mod utils;
mod models;
// public to allow main.rs to use config
pub mod config;

pub fn model(config: &crate::config::Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)
        .expect(&format!("Failed to read from {}", &config.filename))
        .replace("\n", "")
        .replace("\t", "");
    // count threegrams
    let mut counts: HashMap<(char, char, char), i32> = HashMap::new();
    let _ = utils::get_threegram_iter(&content).map(|ngram| {
        let count = counts.entry(ngram).or_insert(0);
        *count += 1;
        ngram})
        .collect::<Vec<_>>();    // collect ends mutable borrow of 'counts' and is necessary therefor
    // calculate probabilities
    let mut model = crate::models::LanguageModel::new(
        &config
            .modelname
            .as_ref()
            .expect("Modelname")
            ).unwrap();
    utils::get_probalities(&counts, &mut model.model);
    model.write_probabilities_to_file(&config.outpath.as_ref().unwrap()[..])
        .expect(
            &format!(
                "Failed to write to {}",
                &config
                .outpath
                .as_ref()
                .expect("Outpath")));
    Ok(())
}

pub fn guess(_config: &crate::config::Config) -> Result<(), Box<dyn Error>> {
    let path =  Path::new("./data/models/");
    let mut models: Vec<crate::models::LanguageModel> = Vec::new();
    let mut model_paths = Vec::new();
    let _ret = match utils::get_model_paths(&path, &mut model_paths) {
        Ok(v) => v,
        Err(e) => panic!("Unrecoverable error while reading model files: {}", e)
    };
    match utils::read_models_from_file(&model_paths, &mut models) {
        Ok(models) => models,
        Err(_) => panic!("Todo")
    };
    for model in models {
        println!("Model name: {:?}", model.name);
    };
    println!("I have to guess!");
    Ok(())
}
