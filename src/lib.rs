extern crate clap;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
//use std::process;


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

fn write_counts_to_file(counts: &HashMap<(char, char, char), i32>, config: &Config) -> std::io::Result<()> {
    let mut write_buf = String::new();
    for ((lhs, mid, rhs), count) in counts {
        write_buf.push_str(
            &format!("{}{}{}\t{}", lhs, mid, rhs, count));
        write_buf.push_str(&String::from("\n"));
    };
    fs::write(&config.modelname, &write_buf)
}

pub fn model(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut counts: HashMap<(char, char, char), i32> = HashMap::new();
    let content = fs::read_to_string(&config.filename)
        .expect(&format!("Failed to read from {}", &config.filename))
        .replace("\n", "");
    let _ = get_threegram_iter(&content).map(|ngram| {
        let count = counts.entry(ngram).or_insert(0);
        *count += 1;
        ngram})
        .collect::<Vec<_>>();    // collect ends mutable borrow of 'counts' and is necessary therefor
    write_counts_to_file(&counts, &config)
        .expect(&format!("Failed to write to {}", &config.modelname));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_letters_by_adapter() {
        let content = "aaa".to_string();
        let mut unigrams = UnigramCounter::new();
        unigrams.count_unigrams(&content[..]);
        match unigrams.counts.get(&'a') {
            Some(count) => assert_eq!(*count, 3),
            None => panic!("The key 'a' is missing in the HashMap!"),
        }
    }

    #[test]
    fn test_count_letters_by_adapter_unseen() {
        let content = "aaa".to_string();
        let mut unigrams = UnigramCounter::new();
        unigrams.count_unigrams(&content[..]);
        assert_eq!(unigrams.counts.get(&'b'), None);
    }

    #[test]
    fn test_count_letters_by_loop() {
        let content = "aaa".to_string();
        let mut unigrams = UnigramCounter::new();
        unigrams.count_unigrams(&content[..]);
        match unigrams.counts.get(&'a') {
            Some(count) => assert_eq!(*count, 3),
            None => panic!("The key 'a' is missing in the HashMap!"),
        }
    }

    #[test]
    fn test_count_letters_by_loop_unseen() {
        let content = "aaa".to_string();
        let mut unigrams = UnigramCounter::new();
        unigrams.count_unigrams(&content[..]);
        assert_eq!(unigrams.counts.get(&'b'), None);
    }

    #[test]
    #[should_panic]
    fn test_division_by_zero() {
        let nominator: i32 = 1;
        let denominator: i32 = 0;
        let unigrams = UnigramCounter::new();
        unigrams.get_probability(&nominator, &denominator);
    }
}
