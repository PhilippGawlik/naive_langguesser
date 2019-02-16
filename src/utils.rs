use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::io;
use regex::Regex;
use std::error::Error;

pub fn get_probability(nominator: &i32, denominator: &i32) -> f32 {
    if *denominator > 0 {
        (*nominator as f32) / (*denominator as f32)
    } else {
        panic!("Division by zero!");
    }
}

pub fn get_probalities(counts: &HashMap<(char, char, char), i32>, probs: &mut HashMap<(char, char, char), f32>) {
    let normalisation_value: i32 = counts.keys().len() as i32;
    for (ngram, c) in counts {
        let prob = probs.entry(*ngram).or_insert(0.0);
        *prob = get_probability(c, &normalisation_value);
    }
}

pub fn get_model_paths(dir: &Path, model_paths: &mut Vec<String>) -> io::Result<()> {
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

pub fn get_threegram_iter<'a>(content: &'a str) -> impl Iterator<Item=(char, char, char)> + 'a {
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

pub fn read_models_from_file(model_paths: &Vec<String>, models: &mut Vec<crate::models::LanguageModel>) -> Result<(), Box<dyn Error>> {
    for path in model_paths {
        models
            .push(crate::models::LanguageModel::from_file(path)
            .unwrap());
    };
    Ok(())
}
