extern crate itertools;

use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::io;
// necesary import for .lines() method of BufReader
use config::Sigma;
use models::errors::InfererError;
use models::errors::LanguageModelError;
use ngram::NgramExt;
use regex::Regex;
use smoothing::{smoothing, SmoothingType};
use std::collections::HashSet;
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use utils::get_model_paths;

pub mod errors;

pub fn get_feature_map(features: &HashSet<String>) -> HashMap<String, i32> {
    features
        .into_iter()
        .map(move |ngram| (ngram.clone(), 0))
        .collect::<HashMap<String, i32>>()
}

pub fn get_features(sigma: &str, n: usize) -> Result<HashSet<String>, LanguageModelError> {
    match n {
        1 => Ok(sigma
            .char_ngrams(1)
            .map(|c| c.to_string())
            .collect::<HashSet<String>>()),
        2 => {
            let sigma_shared = Arc::new(sigma);
            let shared1 = Arc::clone(&sigma_shared);
            let shared2 = Arc::clone(&sigma_shared);
            Ok(shared1
                .char_ngrams(1)
                .cartesian_product(shared2.char_ngrams(1))
                .map(|(a, b)| format!("{}{}", a, b))
                .collect::<HashSet<String>>())
        }
        3 => {
            let sigma_shared = Arc::new(sigma);
            let shared1 = Arc::clone(&sigma_shared);
            let shared2 = Arc::clone(&sigma_shared);
            let shared3 = Arc::clone(&sigma_shared);
            Ok(shared1
                .char_ngrams(1)
                .cartesian_product(
                    shared2
                        .char_ngrams(1)
                        .cartesian_product(shared3.char_ngrams(1)),
                )
                .map(|(a, (b, c))| format!("{}{}{}", a, b, c))
                .collect::<HashSet<String>>())
        }
        _ => Err(LanguageModelError::new(&format!(
            "get_features: Feature size {} not implemented",
            n
        ))),
    }
}

pub struct SimpleModel {
    ngram_length: usize,
    pub model: HashMap<String, i32>,
    rest: i32,
}

impl SimpleModel {
    pub fn new(sigma_ref: &str, n: usize) -> Result<SimpleModel, LanguageModelError> {
        let ngram_length = n;
        let features = get_features(sigma_ref, ngram_length)?;
        let model: HashMap<String, i32> = get_feature_map(&features);
        let rest = 0;
        Ok(SimpleModel {
            ngram_length,
            model,
            rest,
        })
    }

    pub fn add_content(&mut self, content: &str) {
        for ngram in content.char_ngrams(self.ngram_length) {
            if self.model.contains_key(ngram) {
                let mut count = self.model.get_mut(ngram).unwrap();
                *count += 1;
            } else {
                self.rest += 1;
            }
        }
    }

    pub fn get_total_ngram_count(&self) -> i32 {
        self.model.iter().map(|(_, count)| count).sum()
    }

    pub fn get_vocabulary_size(&self) -> usize {
        self.model.iter().count()
    }

    pub fn get_seen_type_count(&self) -> usize {
        self.model.iter().filter(|(_, count)| *count > &0).count()
    }

    pub fn get_unseen_type_count(&self) -> usize {
        self.model.iter().filter(|(_, count)| *count == &0).count()
    }
}

pub struct LanguageModel {
    pub name: String,
    pub model: HashMap<String, f32>,
}

impl LanguageModel {
    pub fn from_str(
        name: &str,
        content: &str,
        sigma_type: Sigma,
        feature_length: usize,
    ) -> Result<LanguageModel, LanguageModelError> {
        let sigma = match sigma_type {
            Sigma::Test => {
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string()
            }
        };
        let mut count_model = SimpleModel::new(&sigma, feature_length)?;
        count_model.add_content(content);
        let probs = smoothing(&count_model, SmoothingType::NoSmoothing)?;
        Ok(LanguageModel {
            name: name.to_string(),
            model: probs,
        })
    }

    pub fn from_name(n: &str) -> Result<LanguageModel, LanguageModelError> {
        let name: String = String::from(n);
        let model: HashMap<String, f32> = HashMap::new();
        return Ok(LanguageModel { name, model });
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
            let mut split = line // looks like: abc\t0.123
                .as_ref() // 'line' needs to outlive 'split'
                .unwrap()
                .split("\t"); // split into: [abc, 0.123]
                              // get ngram as string
            let ngram = String::from(split.next().unwrap());
            // get probability
            let prob = language_model.model.entry(ngram).or_insert(0.0);
            *prob = split.next().unwrap().parse().unwrap();
        }
        Ok(language_model)
    }

    pub fn calculate_str_probability(self, unclassified: &str) -> Result<f32, LanguageModelError> {
        let mut product: f32 = 0.0;
        for ngram in unclassified.char_ngrams(3) {
            product += self.get_ngram_probability(&ngram, 1.0)?.log2().abs();
        }
        Ok(product)
    }

    fn get_ngram_probability(&self, key: &str, default: f32) -> Result<f32, LanguageModelError> {
        if self.model.contains_key(key) {
            return Ok(self.model[key]);
        } else {
            return Ok(default);
        }
    }
    pub fn parse_name_from_path(path: &str) -> Result<String, LanguageModelError> {
        // only build regex once
        lazy_static! {
            static ref get_name: Regex =
                Regex::new(r"\./data/models/([a-zA-Z0-9]+)\.model").unwrap();
        };
        Ok(String::from(
            &get_name
                .captures_iter(path)
                .next()
                .expect("Can't parse model name from path")[1],
        ))
    }

    pub fn write_probabilities_to_file(self, path: &str) -> Result<(), LanguageModelError> {
        let mut write_buf = String::new();
        let _ = self
            .model
            .iter()
            .map(|(ngram, prob)| {
                write_buf.push_str(&format!("{}\t{}", ngram, prob));
                write_buf.push_str(&String::from("\n"));
                (ngram, prob)
            })
            .collect::<Vec<(&String, &f32)>>();
        fs::write(path, &write_buf)?;
        Ok(())
    }
}

pub struct Inferer {
    pub language_models: Vec<LanguageModel>,
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
                    Err(err) => panic!("{} for file {}!", err, &path[..]),
                };
                model
            })
            .collect::<Vec<LanguageModel>>();
        Ok(Inferer { language_models })
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
                    Err(err) => panic!("{} for string {}", err, &unclassified[..]),
                };
                match sender_instance.send((name, probability)) {
                    Ok(_) => drop(sender_instance),
                    Err(err) => panic!(
                        "Thread couldn't send language probability because of {}",
                        err
                    ),
                };
            });
        }
        drop(sender);
        while let Ok(guess) = receiver.recv() {
            prob_table.push(guess);
        }
        Ok(prob_table)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_twogram_features() {
        let sigma = "a💖".to_string();
        let n: usize = 2;
        let features = get_features(&sigma[..], n).unwrap();
        assert_eq!(true, features.contains(&String::from("aa")));
        assert_eq!(true, features.contains(&String::from("a💖")));
        assert_eq!(true, features.contains(&String::from("💖a")));
        assert_eq!(true, features.contains(&String::from("💖💖")));
    }

    #[test]
    fn test_get_threegram_features() {
        let sigma = "ab".to_string();
        let n: usize = 3;
        let features = get_features(&sigma[..], n).unwrap();
        assert_eq!(true, features.contains(&String::from("aaa")));
        assert_eq!(true, features.contains(&String::from("aab")));
        assert_eq!(true, features.contains(&String::from("aba")));
        assert_eq!(true, features.contains(&String::from("abb")));
        assert_eq!(true, features.contains(&String::from("baa")));
        assert_eq!(true, features.contains(&String::from("bab")));
        assert_eq!(true, features.contains(&String::from("bba")));
        assert_eq!(true, features.contains(&String::from("bbb")));
    }

    #[test]
    fn test_get_feature_map() {
        let mut features = HashSet::new();
        features.insert(String::from("aaa"));
        features.insert(String::from("aab"));
        features.insert(String::from("aba"));
        features.insert(String::from("abb"));
        features.insert(String::from("baa"));
        features.insert(String::from("bab"));
        features.insert(String::from("bba"));
        features.insert(String::from("bbb"));
        let feature_map = get_feature_map(&features);
        let keys = feature_map.keys().collect::<HashSet<&String>>();
        for feat in features {
            assert_eq!(keys.contains(&feat), true);
        }
    }

    #[test]
    fn test_add_content_to_simple_model() {
        let sigma = String::from("ab");
        let feature_length = 2;
        let mut simple_model = SimpleModel::new(&sigma, feature_length).unwrap();
        let content = "aabcbba";
        simple_model.add_content(&content);
        assert_eq!(2, simple_model.rest);
        assert_eq!(&1, simple_model.model.get(&String::from("aa")).unwrap());
        assert_eq!(&1, simple_model.model.get(&String::from("ab")).unwrap());
        assert_eq!(&1, simple_model.model.get(&String::from("bb")).unwrap());
        assert_eq!(&1, simple_model.model.get(&String::from("ba")).unwrap());
    }

    #[test]
    fn test_get_total_ngram_counts() {
        let sigma = String::from("ab");
        let feature_length = 2;
        let mut simple_model = SimpleModel::new(&sigma, feature_length).unwrap();
        let content = "aabcbba";
        simple_model.add_content(&content);
        let count: i32 = simple_model.get_total_ngram_count();
        assert_eq!(4, count);
    }

    #[test]
    fn test_get_vocabulary_size() {
        let sigma = String::from("ab");
        let feature_length = 2;
        let mut simple_model = SimpleModel::new(&sigma, feature_length).unwrap();
        let content = "aabcbba";
        simple_model.add_content(&content);
        let count: usize = simple_model.get_vocabulary_size();
        assert_eq!(4, count);
    }

    #[test]
    fn test_get_seen_type_count() {
        let sigma = String::from("ab");
        let feature_length = 2;
        let mut simple_model = SimpleModel::new(&sigma, feature_length).unwrap();
        let content = "abcbba";
        simple_model.add_content(&content);
        let count: usize = simple_model.get_seen_type_count();
        assert_eq!(3, count);
    }

    #[test]
    fn test_get_unseen_type_count() {
        let sigma = String::from("ab");
        let feature_length = 2;
        let mut simple_model = SimpleModel::new(&sigma, feature_length).unwrap();
        let content = "abcbba";
        simple_model.add_content(&content);
        let count: usize = simple_model.get_unseen_type_count();
        assert_eq!(1, count);
    }
    //#[test]
    //#[should_panic]
    //fn test_to_short_for_threegram() {
    //let content = "a".to_string();
    //let _ = get_threegram_iter(&content);
    //}

    //#[test]
    //fn test_get_probability() {
    //let nominator: i32 = 2;
    //let denominator: i32 = 2;
    //let correct_result: f32 = 1.0;
    //assert_eq!(get_probability(&nominator, &denominator), correct_result);
    //}
    //#[test]
    //#[should_panic]
    //fn test_get_probability_division_by_0() {
    //let nominator: i32 = 2;
    //let denominator: i32 = 0;
    //get_probability(&nominator, &denominator);
    //}
    //#[test]
    //fn test_sort_by_second_element() {
    //let mut sort_me: Vec<(String, i32)> = vec![
    //(String::from("a"), 1),
    //(String::from("b"), 2),
    //(String::from("c"), 3),
    //];
    //let sort_me = sort_by_second_element(sort_me);
    //let mut iter = sort_me.into_iter();
    //assert_eq!(iter.next(), Some(("c", 3)));
    //assert_eq!(iter.next(), Some(("b", 2)));
    //assert_eq!(iter.next(), Some(("a", 1)));
    //assert_eq!(iter.next(), None);
    //}
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
