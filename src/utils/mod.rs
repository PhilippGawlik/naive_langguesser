use std::collections::HashMap;
//use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::path::Path;
use utils::errors::UtilError;

pub mod errors;

//pub fn get_threegram_iter<'a>(content: &'a str) -> impl Iterator<Item = (char, char, char)> + 'a {
//if content.chars().count() < 3 {
//panic!("Language text example is too short. Minimum is 3 letters.");
//};
//let mut iter = content.chars();
//// with the deep-copy (char type) ownership is taken
//// unwrap ok, cause text length is checked above
//let mut buf_lhs: char = iter.next().unwrap();
//let mut buf_mid: char = iter.next().unwrap();
//// need the move keyword to also take ownership of rhs
//iter.map(move |rhs| {
//let lhs = buf_lhs;
//let mid = buf_mid;
//buf_lhs = buf_mid;
//buf_mid = rhs;
//(lhs, mid, rhs)
//})
//}

pub fn get_probability(nominator: i32, denominator: i32) -> Result<f32, UtilError> {
    if denominator > 0 {
        Ok((nominator as f32) / (denominator as f32))
    } else {
        Err(UtilError::new(
            "get_probability: Division by zero or negative number!",
        ))
    }
}

pub fn get_probalities(counts: &HashMap<String, i32>) -> Result<HashMap<String, f32>, UtilError> {
    let normalisation_value: i32 = counts.keys().len() as i32;
    let probs: HashMap<String, f32> = counts
        .iter()
        .map(|(ngram, c)| {
            (
                ngram.clone(),
                match get_probability(*c, normalisation_value) {
                    Ok(val) => val,
                    Err(err) => panic!("Got error when calculationg the ngram probability {}", err),
                },
            )
        })
        .collect();
    Ok(probs)
}

pub fn get_model_paths(dir: &Path) -> Result<Vec<String>, UtilError> {
    let mut model_paths = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                // just check the paths on current folder level
                continue;
            } else {
                let path_str = String::from(
                    path.to_str()
                        // Option to Result type
                        .ok_or(UtilError::new(
                            "get_model_paths: Can't convert path to string.",
                        ))?,
                );
                // only build regex once
                lazy_static! {
                    static ref is_model: Regex =
                        Regex::new(r".*\.model").expect("get_model_path: Can't initialise regex.");
                }
                if is_model.is_match(&path_str[..]) {
                    model_paths.push(String::from(
                        path.to_str()
                            // Option to Result type
                            .ok_or(UtilError::new(
                                "get_model_paths: Can't convert path to string.",
                            ))?,
                    ));
                }
            }
        }
    };
    Ok(model_paths)
}

pub fn sort_by_probability(mut vec: Vec<(String, f32)>) -> Result<Vec<(String, f32)>, UtilError> {
    vec.sort_by(|elem1, elem2| { elem1.1.partial_cmp(&elem2.1).unwrap() }.reverse());
    Ok(vec)
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
        assert_eq!(get_probability(&nominator, &denominator), correct_result);
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
