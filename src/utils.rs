use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::io;
use regex::Regex;
use std::error::Error;

use models::LanguageModel;


pub fn get_threegram_iter<'a>(content: &'a str) -> impl Iterator<Item=(char, char, char)> + 'a {
    let mut iter = content.chars();
    let mut buf_lhs: char = match iter.next() {
        Some(c) => c,
        None => panic!("Language example is too short"),
    };
    let mut buf_mid: char = match iter.next() {
        Some(c) => c,
        None => panic!("Language example is too short"),
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

pub fn read_models_from_file(model_paths: &Vec<String>, models: &mut Vec<LanguageModel>) -> Result<(), Box<dyn Error>> {
    for path in model_paths {
        models
            .push(LanguageModel::from_file(path)
            .unwrap());
    };
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
