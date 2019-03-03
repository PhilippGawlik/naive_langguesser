use std::collections::HashMap;
use std::fs;
use std::io;
// necesary import for .lines() method of BufReader
use std::io::prelude::*;
use regex::Regex;
use std::path::Path;
use utils::{
    get_threegram_iter,
    get_probalities,
    get_model_paths
};
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use models::errors::LanguageModelError;
use models::errors::InfererError;
use std::sync::Arc;
//use std::time::Duration;

pub mod errors;

pub struct LanguageModel {
    pub name: String,
    pub model: HashMap<(char, char, char), f32>
}


impl LanguageModel {
    pub fn from_str(n: &str, content: &str) -> Result<LanguageModel, LanguageModelError> {
        let mut counts: HashMap<(char, char, char), i32> = HashMap::new();
        let _ = get_threegram_iter(&content)
            .map(|ngram| {
                let count = counts.entry(ngram).or_insert(0);
                *count += 1;
                ngram})
            .collect::<Vec<_>>();    // collect ends mutable borrow of 'counts' and is necessary therefor
        Ok(LanguageModel{
            name: n.to_string(),
            model: get_probalities(&counts)?
        })
    } 

    pub fn calculate_str_probability(self, unclassified: &str) -> Result<f32, LanguageModelError> {
        let mut product: f32 = 0.0;
        for ngram in get_threegram_iter(unclassified) {
            product += self.get_ngram_probability(&ngram, 1.0)?
                .log2()
                .abs();
        }
        Ok(product)
    }

    fn get_ngram_probability(&self, key: &(char, char, char), default : f32) -> Result<f32, LanguageModelError> {
        if self.model.contains_key(key) {
            return Ok(self.model[key]);
        } else {
            return Ok(default);
        }
    }

    pub fn from_name(n: &str) -> Result<LanguageModel, LanguageModelError> {
        let name: String = String::from(n);
        let model: HashMap<(char, char, char), f32> = HashMap::new();
        return Ok(LanguageModel{name, model})
    }

    pub fn from_file(path: &str) -> Result<LanguageModel, LanguageModelError> {
        // parse name
        let name = LanguageModel::parse_name_from_path(path)?;
        // init new model
        let mut language_model = LanguageModel::from_name(&name[..])?;
        // parse model from file line by line
        let f = fs::File::open(path)?;
        let reader = io::BufReader::new(f);
        for line in reader.lines() {
            let mut split =
                line          // looks like: abc\t0.123
                .as_ref()     // 'line' needs to outlive 'split'
                .unwrap()
                .split("\t");  // split into: [abc, 0.123] 
            // get ngram as string
            let ngram_str = String::from(
                split
                .next()
                .unwrap()
            );
            // get char iterator of ngrams
            let mut iter = ngram_str.chars();
            // get ngram tuple
            let ngram_tup = (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap());
            // get probability
            let prob = language_model.model.entry(ngram_tup).or_insert(0.0);
            *prob = split
                .next()
                .unwrap()
                .parse()
                .unwrap();
        };
        Ok(language_model)
    }

    pub fn parse_name_from_path(path: &str) -> Result<String, LanguageModelError> {
        // only build regex once
        lazy_static! {
            static ref get_name: Regex = Regex::new(r"\./data/models/([a-zA-Z0-9]+)\.model")
                .unwrap();
        };
        Ok(
            String::from(
                &get_name
                .captures_iter(path)
                .next()
                .expect("Can't parse model name from path")
                [1]
            )
        )
    }

    pub fn write_probabilities_to_file(self, path: &str) -> Result<(), LanguageModelError> {
        let mut write_buf = String::new();
        for ((lhs, mid, rhs), count) in self.model {
            write_buf.push_str(&format!("{}{}{}\t{}", lhs, mid, rhs, count));
            write_buf.push_str(&String::from("\n"));
        };
        fs::write(path, &write_buf)?;
        Ok(())
    }
}


pub struct Inferer {
    pub language_models: Vec<LanguageModel>
}

impl Inferer {
    pub fn from_models_path(path: &Path) -> Result<Inferer, InfererError> {
        let model_paths = get_model_paths(&path)?;
        Inferer::from_model_files(model_paths)
    }

    pub fn from_model_files(model_paths: Vec<String>) -> Result<Inferer, InfererError> {
        let language_models = model_paths
            .into_iter()
            .map(|path| {
                let model = match LanguageModel::from_file(&path[..]) {
                    Ok(model) => model,
                    Err(err) => panic!("{} for file {}!", err, &path[..])
                };
                model
            }).collect::<Vec<LanguageModel>>();
        Ok(Inferer{language_models})
    }

    pub fn infer(self, unclassified: String) -> Result<Vec<(String, f32)>, InfererError> {
        let shared_unclassified = Arc::new(unclassified);
        let mut prob_table: Vec<(String, f32)> = Vec::new();
        let (sender, receiver): (Sender<(String, f32)>, Receiver<(String, f32)>) = channel();
        for model in self.language_models {
            let sender_instance = sender.clone();
            let unclassified = Arc::clone(&shared_unclassified);
            thread::spawn(move || {
                let name = model.name.clone();
                let probability = match model.calculate_str_probability(&unclassified[..]) {
                    Ok(prob) => prob,
                    Err(err) => panic!("{} for string {}", err, &unclassified[..])
                };
                match sender_instance.send((name, probability)) {
                    Ok(_) => drop(sender_instance),
                    Err(err) => panic!("Thread couldn't send language probability because of {}", err)
                };
            });
        };
        drop(sender);
        while let Ok(guess) = receiver.recv() {
            prob_table.push(guess);
        };
        Ok(prob_table)
    }
} 
