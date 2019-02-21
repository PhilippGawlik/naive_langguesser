extern crate clap;
#[macro_use] extern crate lazy_static;  //compile regex only once in loops
extern crate regex;
extern crate itertools;
use std::error::Error;
use std::path::Path;
use std::fs;
use models::LanguageModel;
use utils::{
    get_model_paths,
    read_models_from_file,
    get_threegram_iter
};

// these mod statements are just for the compiler to include the modules
mod utils;
mod models;
// public so main.rs can use the config
pub mod config;


pub fn model(config: config::Config) -> Result<(), Box<dyn Error>> {
    // get language example
    let content = fs::read_to_string(&config.filename)?
        .replace("\n", "")
        .replace("\t", "");
    // build language model
    let language_model = LanguageModel::from_str(
        config
            .modelname
            .as_ref()
            .expect("Modelname"),
        &content[..]
    ).expect("Error while initializing language model");
    // write language model
    language_model.write_probabilities_to_file(
        &config.outpath.as_ref().unwrap()[..])
        .expect(
            &format!(
                "Failed to write to {}",
                &config
                    .outpath
                    .as_ref()
                    .expect("Outpath")
            )
        );
    Ok(())
}

pub fn guess(config: config::Config) -> Result<(), Box<dyn Error>> {
    let path =  Path::new("./data/models/");
    let model_paths = match get_model_paths(&path) {
        Ok(paths) => paths,
        Err(e) => panic!("Unrecoverable error while reading model files: {}", e)
    };
    let models = match read_models_from_file(&model_paths) {
        Ok(models) => models,
        Err(_) => panic!("Todo")
    };
    // get language example
    let content = fs::read_to_string(&config.filename)?
        .replace("\n", "")
        .replace("\t", "");
    let mut product1: f32 = 0.0;
    let mut product2: f32 = 0.0;
    let mut product3: f32 = 0.0;
    let _ = get_threegram_iter(&content)
        .map(|ngram| {
            product1 += models[0]
                .get_ngram_probability(&ngram, 1.0)
                .unwrap()
                .log2()
                .abs();
            product2 += models[1]
                .get_ngram_probability(&ngram, 1.0)
                .unwrap()
                .log2()
                .abs();
            product3 += models[2]
                .get_ngram_probability(&ngram, 1.0)
                .unwrap()
                .log2()
                .abs();
            ngram})
        .collect::<Vec<_>>();    // collect ends mutable borrow of 'counts' and is necessary therefor
    println!("I have to guess!");
    println!("Model {} with : {}", models[0].name, product1);
    println!("Model {} with : {}", models[1].name, product2);
    println!("Model {} with : {}", models[2].name, product3);
    Ok(())
}
