extern crate clap;

use std::collections::HashMap;
use std::error::Error;
use std::fs;


pub struct Config {
    filename: String,
    modelname: String,
}

impl Config {
    pub fn new(matches: &clap::ArgMatches) -> Result<Config, &'static str> {
        let model_matches = matches.subcommand_matches("model").unwrap();
        let modelname = format!(
            "data/models/{}.model",
            model_matches
                .value_of("model-name")
                .unwrap()  // clap ensures existing value
                .to_string());
        let filename = model_matches
            .value_of("path")
            .unwrap()  // clap ensures existing value
            .to_string();
        Ok(Config{filename, modelname})
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

fn write_probabilities_to_file(counts: &HashMap<(char, char, char), f32>, config: &Config) -> std::io::Result<()> {
    let mut write_buf = String::new();
    for ((lhs, mid, rhs), count) in counts {
        write_buf.push_str(
            &format!("{}{}{}\t{}", lhs, mid, rhs, count));
        write_buf.push_str(&String::from("\n"));
    };
    fs::write(&config.modelname, &write_buf)
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
        .replace("\n", "");
    // count threegrams
    let mut counts: HashMap<(char, char, char), i32> = HashMap::new();
    let _ = get_threegram_iter(&content).map(|ngram| {
        let count = counts.entry(ngram).or_insert(0);
        *count += 1;
        ngram})
        .collect::<Vec<_>>();    // collect ends mutable borrow of 'counts' and is necessary therefor
    // calculate probabilities
    let mut probs: HashMap<(char, char, char), f32> = HashMap::new();
    get_probalities(&counts, &mut probs);
    write_probabilities_to_file(&probs, &config)
        .expect(&format!("Failed to write to {}", &config.modelname));
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
