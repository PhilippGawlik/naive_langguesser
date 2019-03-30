//use std::collections::HashMap;
//use itertools::Itertools;
//use ngram::NgramExt;
use regex::Regex;
use std::fs;
use std::path::Path;
use utils::errors::UtilError;

pub mod errors;

pub fn get_probability(nominator: f32, denominator: f32) -> Result<f32, UtilError> {
    if denominator > 0.0 {
        Ok(nominator / (denominator as f32))
    } else {
        Err(UtilError::new(
            "get_probability: Division by zero or negative number!",
        ))
    }
}

//pub fn count_ngrams(string: &str, n: usize) -> Result<HashMap<String, i32>, UtilError> {
//let mut counts: HashMap<String, i32> = HashMap::new();
//let _ = string
//.char_ngrams(n)
//.map(|ngram| {
//let count = counts.entry(ngram.to_string()).or_insert(0);
//*count += 1;
//ngram
//})
//.collect::<Vec<_>>();
//Ok(counts)
//}

//pub fn calc_probalities(counts: &HashMap<String, i32>) -> Result<HashMap<String, f32>, UtilError> {
//let mut out: Vec<(String, i32)> = counts.iter().map(|(s, f)| (s.clone(), f.clone())).collect();
//out = sort_by_second_element(out)
//.unwrap()
//.into_iter()
//.take(99)
//.collect::<Vec<_>>();
//let normalisation_value: i32 = out.iter().map(|(_, count)| count).sum();
//Ok(out
//.iter()
//.map(|(ngram, c)| {
//(
//ngram.clone(),
//match get_probability(*c as f32, normalisation_value as f32) {
//Ok(val) => val,
//Err(err) => panic!("Got error when calculationg the ngram probability {}", err),
//},
//)
//})
//.collect::<HashMap<String, f32>>())
//}

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

pub fn sort_by_second_element<T: PartialOrd>(
    mut vec: Vec<(String, T)>,
) -> Result<Vec<(String, T)>, UtilError> {
    vec.sort_by(|tuple1, tuple2| { tuple1.1.partial_cmp(&tuple2.1).unwrap() }.reverse());
    Ok(vec)
}

#[cfg(test)]
mod test {
    use super::*;

    //#[test]
    //fn test_get_threegram_iter() {
    //let content = "abcd".to_string();
    //let mut iter = get_threegram_iter(&content);
    //assert_eq!(iter.next(), Some(('a', 'b', 'c')));
    //assert_eq!(iter.next(), Some(('b', 'c', 'd')));
    //assert_eq!(iter.next(), None);
    //}
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
