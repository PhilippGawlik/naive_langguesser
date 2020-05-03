pub mod count_model;
pub mod errors;
pub mod ngram_model;
pub mod probability_model;
pub mod sigma;
pub mod text_model;

use itertools::Itertools; // for cartesian_product
use models::sigma::Sigma;
use models::text_model::TextModel;
use ngram::NgramExt;

struct NGramGenerator {
    unigrams: Vec<String>,
}

impl NGramGenerator {
    pub fn generate(&self, ngram_length: usize) -> Vec<String> {
        self.recursion(self.unigrams.clone(), ngram_length - 1)
    }

    fn recursion(&self, ngrams: Vec<String>, index: usize) -> Vec<String> {
        match index {
            0 => ngrams,
            _ => {
                let unigrams: &Vec<String> = &self.unigrams;
                let ext_ngrams: Vec<String> = ngrams
                    .iter()
                    .cartesian_product(unigrams.iter())
                    .map(|(ngram, unigram)| format!("{}{}", ngram, unigram))
                    .collect::<Vec<String>>();
                self.recursion(ext_ngrams, index - 1)
            }
        }
    }
}

pub trait NGramExt {
    fn string_ngrams(&self, n: usize) -> Vec<String>;
}

impl NGramExt for Sigma {
    fn string_ngrams(&self, ngram_length: usize) -> Vec<String> {
        assert!(ngram_length > 0);
        let generator = NGramGenerator {
            unigrams: self.as_vec(),
        };
        generator.generate(ngram_length)
    }
}

pub fn get_ngrams(text: &str, n: usize) -> Vec<String> {
    text.char_ngrams(n)
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
}

impl NGramExt for TextModel {
    fn string_ngrams(&self, ngram_length: usize) -> Vec<String> {
        let text: String = self.get_text();
        assert!(ngram_length > 0);
        get_ngrams(&text[..], ngram_length)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use models::sigma::SigmaType;

    #[test]
    fn test_get_unigram_features() {
        let sigma: Sigma = Sigma::new(None, SigmaType::Test);
        let mut ngrams = sigma.string_ngrams(1);
        let mut result: Vec<String> = Vec::new();
        result.push(String::from("a"));
        result.push(String::from("b"));
        result.push(String::from("c"));
        ngrams.sort();
        result.sort();
        assert_eq!(result, ngrams);
    }

    #[test]
    fn test_get_bigram_features() {
        let sigma: Sigma = Sigma::new(None, SigmaType::Test);
        let mut ngrams = sigma.string_ngrams(2);
        let mut result: Vec<String> = Vec::new();
        result.push(String::from("aa"));
        result.push(String::from("ab"));
        result.push(String::from("ac"));
        result.push(String::from("ba"));
        result.push(String::from("bb"));
        result.push(String::from("bc"));
        result.push(String::from("ca"));
        result.push(String::from("cb"));
        result.push(String::from("cc"));
        ngrams.sort();
        result.sort();
        assert_eq!(result, ngrams);
    }

    #[test]
    fn test_get_threegram_features() {
        let sigma: Sigma = Sigma::new(None, SigmaType::Test);
        let mut ngrams = sigma.string_ngrams(3);
        let mut result: Vec<String> = Vec::new();
        result.push(String::from("aaa"));
        result.push(String::from("aab"));
        result.push(String::from("aac"));
        result.push(String::from("cbc"));
        result.push(String::from("bcc"));
        result.push(String::from("bbc"));
        result.push(String::from("bba"));
        result.push(String::from("acc"));
        result.push(String::from("bbb"));
        result.push(String::from("bab"));
        result.push(String::from("abb"));
        result.push(String::from("aca"));
        result.push(String::from("baa"));
        result.push(String::from("ccc"));
        result.push(String::from("cca"));
        result.push(String::from("cba"));
        result.push(String::from("ccb"));
        result.push(String::from("aba"));
        result.push(String::from("bcb"));
        result.push(String::from("abc"));
        result.push(String::from("cac"));
        result.push(String::from("cbb"));
        result.push(String::from("caa"));
        result.push(String::from("bca"));
        result.push(String::from("cab"));
        result.push(String::from("bac"));
        result.push(String::from("acb"));
        ngrams.sort();
        result.sort();
        assert_eq!(result, ngrams);
    }
}
