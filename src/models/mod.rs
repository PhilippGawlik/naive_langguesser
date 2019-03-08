use std::collections::HashMap;
use std::fs;
use std::io;
// necesary import for .lines() method of BufReader
use models::errors::InfererError;
use models::errors::LanguageModelError;
use regex::Regex;
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use utils::{get_model_paths, get_probalities};
//use std::time::Duration;
use std::iter;
use std::slice;

pub mod errors;

// warum kein Result?
#[inline]
fn char_width(byte: u8) -> usize {
    // why not make it [usize; size] to spare one cast?
    // why not make vector global?
    const TABLE: [u8; 16] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 3, 4];
    TABLE[(byte >> 4) as usize] as usize
}

// warum kein Result?
#[inline]
fn char_offsets(text: &str) -> CharOffsets {
    CharOffsets {
        iter: text.as_bytes().iter(),
        step: 0,
        offset: 0,
    }
}

#[derive(Clone, Debug)]
struct CharOffsets<'a> {
    iter: slice::Iter<'a, u8>,
    step: usize,
    offset: usize,
}

impl<'a> Iterator for CharOffsets<'a> {
    type Item = usize;

    #[inline]
    // where is the Some for return value?
    fn next(&mut self) -> Option<usize> {
        // iter is not assign, other struct values are, why?
        self.iter.nth(self.step).map(|&byte| {
            let width = char_width(byte);
            self.step = width - 1;
            let current_offset = self.offset;
            self.offset += width;
            // should be Some(current_offset)
            current_offset
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.iter.len();
        ((length + 3) / 4, Some(length))
    }
}

#[derive(Debug)]
pub struct CharNgrams<'a> {
    text: &'a str,
    starts: CharOffsets<'a>,
    ends: iter::Skip<CharOffsets<'a>>,
    finished: bool,
}

impl<'a> CharNgrams<'a> {
    #[inline]
    fn next_span(&mut self) -> Option<(usize, usize)> {
        if self.finished {
            return None;
        }

        let end = match self.ends.next() {
            Some(end) => end,
            None => {
                self.finished = true;
                self.text.len()
            }
        };
        self.starts.next().map(|start| (start, end))
    }
}

impl<'a> Iterator for CharNgrams<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        self.next_span().map(|(start, end)| &self.text[start..end])
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.ends.size_hint();
        (lower, upper.map(|x| x + 1))
    }
}

#[derive(Debug)]
pub struct CharNgramIndices<'a>(CharNgrams<'a>);

impl<'a> Iterator for CharNgramIndices<'a> {
    type Item = (usize, &'a str);

    #[inline]
    fn next(&mut self) -> Option<(usize, &'a str)> {
        self.0
            .next_span()
            .map(|(start, end)| (start, &self.0.text[start..end]))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
pub trait NgramExt {
    fn char_ngrams(&self, n: usize) -> CharNgrams;
    fn char_ngram_indices(&self, n: usize) -> CharNgramIndices;
}

impl NgramExt for str {
    fn char_ngrams(&self, n: usize) -> CharNgrams {
        assert!(n > 0);
        let starts = char_offsets(self);
        let ends = starts.clone().skip(n);
        CharNgrams {
            text: &self,
            starts,
            ends,
            finished: false,
        }
    }

    fn char_ngram_indices(&self, n: usize) -> CharNgramIndices {
        CharNgramIndices(self.char_ngrams(n))
    }
}

pub struct LanguageModel {
    pub name: String,
    pub model: HashMap<String, f32>,
}

impl LanguageModel {
    pub fn from_str(n: &str, content: &str) -> Result<LanguageModel, LanguageModelError> {
        let mut counts: HashMap<String, i32> = HashMap::new();
        let _ = &content
            .char_ngrams(3)
            .map(|ngram| {
                let count = counts.entry(ngram.to_string()).or_insert(0);
                *count += 1;
                ngram
            })
            .collect::<Vec<_>>(); // collect ends mutable borrow of 'counts' and is necessary therefor
        Ok(LanguageModel {
            name: n.to_string(),
            model: get_probalities(&counts)?,
        })
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
        for (ngram, count) in self.model {
            write_buf.push_str(&format!("{}\t{}", ngram, count));
            write_buf.push_str(&String::from("\n"));
        }
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
