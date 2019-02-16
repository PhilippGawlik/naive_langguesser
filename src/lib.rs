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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_threegram_iter() {
        let content = "abcd".to_string();
        let mut iter = get_threegram_iter(&content);
        assert_eq!(iter.next(), Some(('a', 'b', 'c')));
        assert_eq!(iter.next(), Some(('b', 'c', 'd')));
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[should_panic]
    fn test_to_short_for_threegram() {
        let content = "a".to_string();
        let _ = get_threegram_iter(&content);
    }


    #[test]
    fn test_get_probability() {
        let nominator: i32 = 2;
        let denominator: i32 = 2;
        let correct_result: f32 = 1.0;
        assert_eq!(
            get_probability(&nominator, &denominator),
            correct_result);
    }

    #[test]
    #[should_panic]
    fn test_get_probability_division_by_0() {
        let nominator: i32 = 2;
        let denominator: i32 = 0;
        get_probability(&nominator, &denominator);
    }

    //#[test]
    //fn test_count_letters_by_loop() {
        //let content = "aaa".to_string();
        //let mut unigrams = UnigramCounter::new();
        //unigrams.count_unigrams(&content[..]);
        //match unigrams.counts.get(&'a') {
            //Some(count) => assert_eq!(*count, 3),
            //None => panic!("The key 'a' is missing in the HashMap!"),
        //}
    //}
}
