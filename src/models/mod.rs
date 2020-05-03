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


#[cfg(test)]
mod test {
    use super::*;
    use models::sigma::SigmaType;


    #[test]
    fn test_get_unigram_features() {
        let sigma: Sigma = Sigma::new(None, SigmaType::Test);
        let ngrams = sigma.string_ngrams(1);
        let mut result: HashSet<String> = HashSet::new();
        result.insert(String::from("a"));
        result.insert(String::from("b"));
        result.insert(String::from("c"));
        assert_eq!(result, ngrams);
    }

    #[test]
    fn test_get_bigram_features() {
        let sigma: Sigma = Sigma::new(None, SigmaType::Test);
        let ngrams = sigma.string_ngrams(2);
        let mut result: HashSet<String> = HashSet::new();
        result.insert(String::from("aa"));
        result.insert(String::from("ab"));
        result.insert(String::from("ac"));
        result.insert(String::from("ba"));
        result.insert(String::from("bb"));
        result.insert(String::from("bc"));
        result.insert(String::from("ca"));
        result.insert(String::from("cb"));
        result.insert(String::from("cc"));
        assert_eq!(result, ngrams);
    }

    #[test]
    fn test_get_threegram_features() {
        let sigma: Sigma = Sigma::new(None, SigmaType::Test);
        let ngrams = sigma.string_ngrams(3);
        let mut result: HashSet<String> = HashSet::new();
        result.insert(String::from("cbc"));
        result.insert(String::from("bcc"));
        result.insert(String::from("bbc"));
        result.insert(String::from("bba"));
        result.insert(String::from("aab"));
        result.insert(String::from("acc"));
        result.insert(String::from("bbb"));
        result.insert(String::from("bab"));
        result.insert(String::from("abb"));
        result.insert(String::from("aca"));
        result.insert(String::from("baa"));
        result.insert(String::from("aaa"));
        result.insert(String::from("ccc"));
        result.insert(String::from("cca"));
        result.insert(String::from("cba"));
        result.insert(String::from("ccb"));
        result.insert(String::from("aba"));
        result.insert(String::from("bcb"));
        result.insert(String::from("abc"));
        result.insert(String::from("aac"));
        result.insert(String::from("cac"));
        result.insert(String::from("cbb"));
        result.insert(String::from("caa"));
        result.insert(String::from("bca"));
        result.insert(String::from("cab"));
        result.insert(String::from("bac"));
        result.insert(String::from("acb"));
        assert_eq!(result, ngrams);
    }
}
