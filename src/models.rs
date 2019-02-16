use std::collections::HashMap;
use std::fs;
use std::io;
// necesary import for .lines() method of BufReader
use std::io::prelude::*;
use regex::Regex;


pub struct LanguageModel {
    pub name: String,
    pub model: HashMap<(char, char, char), f32>
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

    pub fn write_probabilities_to_file(self, path: &str) -> std::io::Result<()> {
        let mut write_buf = String::new();
        for ((lhs, mid, rhs), count) in self.model {
            write_buf.push_str(&format!("{}{}{}\t{}", lhs, mid, rhs, count));
            write_buf.push_str(&String::from("\n"));
        };
        fs::write(path, &write_buf)?;
        Ok(())
    }
}
