extern crate clap;
#[macro_use] extern crate lazy_static;  //compile regex only once in loops
extern crate regex;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::io::prelude::*;
use regex::Regex;


pub enum Mode {
    Model,
    Guess
}

pub struct LanguageModel {
    name: String,
    model: HashMap<(char, char, char), f32>
}

impl LanguageModel {
    pub fn new(n: &str) -> Result<LanguageModel, &'static str> {
        let name: String = String::from(n);
        let model: HashMap<(char, char, char), f32> = HashMap::new();
        return Ok(LanguageModel{name, model})
    }

    pub fn from_file(path: &str) -> Result<LanguageModel, &'static str> {
        // parse name
        let name = LanguageModel::parse_name_from_path(path).unwrap();
        // init new model
        let mut language_model = LanguageModel::new(&name[..]).unwrap();
        // parse model from file line by line
        let f = fs::File::open(path).expect("Can't open model file");
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

    pub fn parse_name_from_path(path: &str) -> Result<String, &'static str> {
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

    fn write_probabilities_to_file(self, path: &str) -> std::io::Result<()> {
        let mut write_buf = String::new();
        for ((lhs, mid, rhs), count) in self.model {
            write_buf.push_str(&format!("{}{}{}\t{}", lhs, mid, rhs, count));
            write_buf.push_str(&String::from("\n"));
        };
        fs::write(path, &write_buf)
    }
}

pub struct Config {
    filename: String,
    modelname: Option<String>,
    outpath: Option<String>,
    pub application_mode: Mode
}

impl Config {
    pub fn new(matches: &clap::ArgMatches) -> Result<Config, &'static str> {
        if let Some(model_matches) = matches.subcommand_matches("model") {
            let filename = model_matches
                .value_of("path")
                .unwrap()  // clap ensures existing value
                .to_string();
            let modelname = Some(model_matches
                .value_of("model-name")
                .unwrap()  // clap ensures existing value
                .to_string());
            let outpath = Some(format!( "data/models/{}.model", model_matches
                .value_of("model-name")
                .unwrap()  // clap ensures existing value
                .to_string()));
            let application_mode = Mode::Model;
            return Ok(Config{filename, modelname, outpath, application_mode})
        } else {
            let model_matches = matches.subcommand_matches("guess").unwrap();
            let filename = model_matches
                .value_of("path")
                .unwrap()  // clap ensures existing value
                .to_string();
            let modelname = None;
            let outpath = None;
            let application_mode = Mode::Guess;
            return Ok(Config{filename, modelname, outpath, application_mode})
        };
    }
}

fn get_threegram_iter<'a>(content: &'a str) -> impl Iterator<Item=(char, char, char)>  + 'a {
    let mut iter = content.chars();
    let mut buf_lhs: char = match iter.next() {
        Some(c) => c,
        None => panic!("String is too short"),
    };
    let mut buf_mid: char = match iter.next() {
        Some(c) => c,
        None => panic!("String is too short"),
    };
    iter.map(
        move |rhs| {
            let lhs = buf_lhs;
            let mid = buf_mid;
            buf_lhs = buf_mid;
            buf_mid = rhs;
            (lhs, mid, rhs)
        }
    )
}

fn get_probalities(counts: &HashMap<(char, char, char), i32>, probs: &mut HashMap<(char, char, char), f32>) {
    let normalisation_value: i32 = counts.keys().len() as i32;
    for (ngram, c) in counts {
        let prob = probs.entry(*ngram).or_insert(0.0);
        *prob = get_probability(c, &normalisation_value);
    }
}

fn get_probability(nominator: &i32, denominator: &i32) -> f32 {
    if *denominator > 0 {
        (*nominator as f32) / (*denominator as f32)
    } else {
        panic!("Division by zero!");
    }
}

pub fn model(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)
        .expect(&format!("Failed to read from {}", &config.filename))
        .replace("\n", "")
        .replace("\t", "");
    // count threegrams
    let mut counts: HashMap<(char, char, char), i32> = HashMap::new();
    let _ = get_threegram_iter(&content).map(|ngram| {
        let count = counts.entry(ngram).or_insert(0);
        *count += 1;
        ngram})
        .collect::<Vec<_>>();    // collect ends mutable borrow of 'counts' and is necessary therefor
    // calculate probabilities
    let mut model = LanguageModel::new(
        &config
            .modelname
            .as_ref()
            .expect("Modelname")
            ).unwrap();
    get_probalities(&counts, &mut model.model);
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

fn get_model_paths(dir: &Path, model_paths: &mut Vec<String>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry.unwrap().path();
            if path.is_dir() {
                // just check the paths on current folder level
                continue;
            } else {
                let path_str = String::from(path.to_str().unwrap());
                // only build regex once
                lazy_static! {
                        static ref is_model: Regex = Regex::new(r".*\.model").unwrap();
                    }
                if is_model.is_match(&path_str[..]) {
                    model_paths.push(String::from(path.to_str().unwrap()));
                }
            }
        }
    }
    Ok(())
}

fn read_models_from_file(model_paths: &Vec<String>, models: &mut Vec<LanguageModel>) -> Result<(), Box<dyn Error>> {
    for path in model_paths {
        models
            .push(LanguageModel::from_file(path)
            .unwrap());
    };
    Ok(())
}

pub fn guess(_config: &Config) -> Result<(), Box<dyn Error>> {
    let path =  Path::new("./data/models/");
    let mut models: Vec<LanguageModel> = Vec::new();
    let mut model_paths = Vec::new();
    let _ret = match get_model_paths(&path, &mut model_paths) {
        Ok(v) => v,
        Err(e) => panic!("Unrecoverable error while reading model files: {}", e)
    };
    match read_models_from_file(&model_paths, &mut models) {
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
