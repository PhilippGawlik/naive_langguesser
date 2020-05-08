use itertools::Itertools;
use models::symbol::Symbol;
use std::collections::HashSet;


#[derive(Clone)]
pub enum SigmaType {
    AlphaNum,
    Ascii,
    Test,
}

impl SigmaType {
    pub fn get_symbols(&self) -> HashSet<Symbol> {
        match self {
            SigmaType::AlphaNum => {
                let numbers: HashSet<Symbol> = (48..=57)
                    .into_iter()
                    .map(|byte| Symbol::from_u8(byte))
                    .collect();
                let capitals: HashSet<Symbol> = (65..=90)
                    .into_iter()
                    .map(|byte| Symbol::from_u8(byte))
                    .collect();
                let lower: HashSet<Symbol> = (97..=122)
                    .into_iter()
                    .map(|byte| Symbol::from_u8(byte))
                    .collect();
                let mut sigma: HashSet<Symbol> = HashSet::new();
                sigma.extend(numbers);
                sigma.extend(capitals);
                sigma.extend(lower);
                sigma
            }
            SigmaType::Ascii => (0..=127)
                .into_iter()
                .map(|byte| Symbol::from_u8(byte))
                .collect(),
            SigmaType::Test => (97..=99)
                .into_iter()
                .map(|byte| Symbol::from_u8(byte))
                .collect(),
        }
    }
}

#[derive(Clone)]
pub struct Sigma {
    pub set_marker: Option<Symbol>,
    pub sigma_type: SigmaType,
    pub sigma: HashSet<Symbol>,
}

impl Sigma {
    pub fn new(set_marker_byte: Option<u8>, sigma_type: SigmaType) -> Sigma {
        let mut sigma: HashSet<Symbol> = sigma_type.get_symbols();
        let set_marker: Option<Symbol> = match set_marker_byte {
            Some(marker_byte) => {
                sigma.insert(Symbol::from_u8(marker_byte));
                Some(Symbol::from_u8(marker_byte))
            }
            None => None,
        };
        Sigma {
            set_marker,
            sigma,
            sigma_type,
        }
    }

    pub fn contains(&self, symbol: Symbol) -> Option<Symbol> {
        match self.sigma.contains(&symbol) {
            true => Some(symbol.clone()),
            false => None,
        }
    }

    pub fn as_ref(&self) -> &HashSet<Symbol> {
        &self.sigma
    }

    pub fn as_set(&self) -> HashSet<Symbol> {
        self.sigma
            .iter()
            .map(|symbol| symbol.clone())
            .collect::<HashSet<Symbol>>()
    }

    pub fn as_vec(&self) -> Vec<Symbol> {
        self.sigma
            .iter()
            .map(|symbol| symbol.clone())
            .collect::<Vec<Symbol>>()
    }

    pub fn as_string_vec(&self) -> Vec<String> {
        self.sigma
            .iter()
            .map(|symbol| symbol.as_string())
            .collect::<Vec<String>>()
    }
}

pub struct NGramGenerator {
    pub unigrams: Vec<String>,
}

impl NGramGenerator {
    pub fn generate(&self, ngram_length: usize) -> Vec<String> {
        self.recursion(self.unigrams.clone(), ngram_length - 1)
    }

    pub fn recursion(&self, ngrams: Vec<String>, index: usize) -> Vec<String> {
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
    fn ngrams(&self, n: usize) -> Vec<String>;
}

impl NGramExt for Sigma {
    fn ngrams(&self, ngram_length: usize) -> Vec<String> {
        assert!(ngram_length > 0);
        let generator = NGramGenerator {
            unigrams: self.as_string_vec(),
        };
        generator.generate(ngram_length)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_symbol1() {
        let lhs = Symbol::from_u8(97);
        let rhs = Symbol::from_u8(97);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_symbol2() {
        let lhs = Symbol::from_u8(97);
        let rhs = Symbol::from_u8(98);
        assert_ne!(lhs, rhs);
    }

    #[test]
    fn test_get_unigram_features() {
        let sigma: Sigma = Sigma::new(None, SigmaType::Test);
        let mut ngrams = sigma.ngrams(1);
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
        let mut ngrams = sigma.ngrams(2);
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
        let mut ngrams = sigma.ngrams(3);
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
