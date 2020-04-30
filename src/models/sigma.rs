use itertools::Itertools; // cartesian_product
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Clone)]
pub enum SigmaType {
    AlphaNum,
    Ascii,
    Test,
}

#[derive(Clone)]
pub struct Sigma {
    pub set_marker: Option<u8>,
    sigma: HashSet<u8>,
    type_: SigmaType,
}

impl Sigma {
    pub fn new(set_marker: Option<u8>, type_: SigmaType) -> Sigma {
        let mut sigma: HashSet<u8> = match type_ {
            SigmaType::AlphaNum => {
                let numbers: HashSet<u8> = (48..=57).into_iter().collect();
                let capitals: HashSet<u8> = (65..=90).into_iter().collect();
                let lower: HashSet<u8> = (97..=122).into_iter().collect();
                let mut sigma: HashSet<u8> = HashSet::new();
                sigma.extend(numbers);
                sigma.extend(capitals);
                sigma.extend(lower);
                sigma
            }
            SigmaType::Ascii => (0..=127).into_iter().collect(),
            SigmaType::Test => (97..=99).into_iter().collect(),
        };
        match set_marker {
            Some(byte) => {
                sigma.insert(byte)
            },
            None => false
        };
        Sigma {set_marker, sigma, type_}
    }

    pub fn contains(&self, byte: &u8) -> Option<u8> {
        match self.sigma.contains(byte) {
            true => Some(byte.clone()),
            false => None,
        }
    }

    pub fn as_bytes(&self) -> &HashSet<u8> {
        &self.sigma
    }
}


pub trait NGramExt {
    fn string_ngrams(&self, n: usize) -> HashSet<String>;
}

impl NGramExt for Sigma {
    fn string_ngrams(&self, ngram_length: usize) -> HashSet<String> {
        match ngram_length {
            1 => self
                .as_bytes()
                .iter()
                .map(|b| (*b as char).to_string())
                .collect::<HashSet<String>>(),
            2 => {
                let sigma_shared = Arc::new(self.as_bytes());
                let shared1 = Arc::clone(&sigma_shared);
                let shared2 = Arc::clone(&sigma_shared);
                shared1
                    .iter()
                    .cartesian_product(shared2.iter())
                    .map(|(a, b)| format!("{}{}", *a as char, *b as char))
                    .collect::<HashSet<String>>()
            }
            3 => {
                let sigma_shared = Arc::new(self.as_bytes());
                let shared1 = Arc::clone(&sigma_shared);
                let shared2 = Arc::clone(&sigma_shared);
                let shared3 = Arc::clone(&sigma_shared);
                shared1
                    .iter()
                    .cartesian_product(shared2.iter().cartesian_product(shared3.iter()))
                    .map(|(a, (b, c))| format!("{}{}{}", *a as char, *b as char, *c as char))
                    .collect::<HashSet<String>>()
            }
            _ => panic!(
                "Trait NGramExt not implemented for length: {}",
                ngram_length
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
