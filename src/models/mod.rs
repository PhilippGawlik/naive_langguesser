pub mod count_model;
pub mod errors;
pub mod ngram_model;
pub mod probability_model;
pub mod sigma;
pub mod text_model;

use std::collections::HashSet;
use itertools::Itertools; // for cartesian_product
use models::sigma::Sigma;


struct NGramGenerator {
    unigrams: HashSet<String>,
}

impl NGramGenerator {
    pub fn generate(&self, ngram_length: usize) -> HashSet<String> {
        self.recursion(self.unigrams.clone(), ngram_length - 1)
    }

    fn recursion(&self, ngrams: HashSet<String>, index: usize) -> HashSet<String> {
        match index {
            0 => ngrams,
            _ => {
                let unigrams: &HashSet<String> = &self.unigrams;
                let ext_ngrams: HashSet<String> = ngrams
                    .iter()
                    .cartesian_product(unigrams.iter())
                    .map(|(ngram, unigram)| format!("{}{}", ngram, unigram))
                    .collect::<HashSet<String>>();
                self.recursion(ext_ngrams, index - 1)
            }
        }
    }
}


pub trait NGramExt {
    fn string_ngrams(&self, n: usize) -> HashSet<String>;
}

impl NGramExt for Sigma {
    fn string_ngrams(&self, ngram_length: usize) -> HashSet<String> {
        assert!(ngram_length > 0);
        let generator = NGramGenerator {
            unigrams: self.as_string(),
        };
        generator.generate(ngram_length)
    }
}
