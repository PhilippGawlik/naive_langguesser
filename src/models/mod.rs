pub mod count_model;
pub mod errors;
pub mod ngram_model;
pub mod probability_model;
pub mod sigma;
pub mod text_model;

use models::ngram_model::NGramGenerator;
use models::sigma::Sigma;
use std::{iter, slice};


#[inline]
pub fn char_width(byte: u8) -> usize {
    const TABLE: [usize; 16] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 3, 4];
    TABLE[(byte >> 4) as usize]
}

#[inline]
pub fn symbol_offsets(text: &str) -> SymbolOffsets {
    SymbolOffsets {
        iter: text.as_bytes().iter(),
        step: 0,
        offset: 0,
    }
}

#[derive(Debug, Clone)]
pub struct SymbolOffsets<'a> {
    iter: slice::Iter<'a, u8>,
    step: usize,
    offset: usize,
}

impl<'a> Iterator for SymbolOffsets<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        self.iter.nth(self.step).map(|&byte| {
            let width = char_width(byte);
            self.step = width - 1;
            let current_offset = self.offset;
            self.offset += width;
            current_offset
        })
    }
}

#[derive(Debug, Clone)]
pub struct SymbolSlices<'a> {
    text: &'a str,
    starts: SymbolOffsets<'a>,
    ends: iter::Skip<SymbolOffsets<'a>>,
    finished: bool,
}

impl<'a> SymbolSlices<'a> {
    #[inline]
    fn next_symbol(&mut self) -> Option<(usize, usize)> {
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

impl<'a> Iterator for SymbolSlices<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        self.next_symbol().map(|(start, end)| &self.text[start..end])
    }

}

pub trait SymbolExt {
    fn get_symbols(&self) -> SymbolSlices;
}

impl SymbolExt for str {
    fn get_symbols(&self) -> SymbolSlices {
        let starts = symbol_offsets(self);
        let ends = starts.clone().skip(1);
        SymbolSlices {
            text: &self,
            starts,
            ends,
            finished: false
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
            unigrams: self
                .as_vec()
                .iter()
                .map(|symbol| symbol.as_string())
                .collect::<Vec<String>>(),
        };
        generator.generate(ngram_length)
    }
}

impl NGramExt for String {
    fn string_ngrams(&self, ngram_length: usize) -> Vec<String> {
        assert!(ngram_length > 0);
        self.get_symbols()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
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

    static CORRECT_CHAR_WIDTH: [u8; 256] = [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    #[test]
    fn test_char_width() {
        for (i, expected_width) in CORRECT_CHAR_WIDTH.iter().enumerate() {
            if *expected_width > 0 {
                assert_eq!(char_width(i as u8), *expected_width as usize);
            }
        }
    }

    #[test]
    fn test_unigrams() {
        assert_eq!("".get_symbols().next(), None);
        let text = "ζoobαr 💖";
        let mut ngrams = text.get_symbols();
        assert_eq!(ngrams.next(), Some("ζ"));
        assert_eq!(ngrams.next(), Some("o"));
        assert_eq!(ngrams.next(), Some("o"));
        assert_eq!(ngrams.next(), Some("b"));
        assert_eq!(ngrams.next(), Some("α"));
        assert_eq!(ngrams.next(), Some("r"));
        assert_eq!(ngrams.next(), Some(" "));
        assert_eq!(ngrams.next(), Some("💖"));
        assert_eq!(ngrams.next(), None);
    }

    //#[test]
    //fn test_bigrams() {
        //assert_eq!("".char_ngrams(2).next(), None);
        //let text = "ζoobαr 💖";
        //let mut ngrams = text.char_ngrams(2);
        //assert_eq!(ngrams.next(), Some("ζo"));
        //assert_eq!(ngrams.next(), Some("oo"));
        //assert_eq!(ngrams.next(), Some("ob"));
        //assert_eq!(ngrams.next(), Some("bα"));
        //assert_eq!(ngrams.next(), Some("αr"));
        //assert_eq!(ngrams.next(), Some("r "));
        //assert_eq!(ngrams.next(), Some(" 💖"));
        //assert_eq!(ngrams.next(), None);
    //}

    //#[test]
    //fn test_trigrams() {
        //assert_eq!("".char_ngrams(3).next(), None);
        //let text = "ζoobαr 💖";
        //let mut ngrams = text.char_ngrams(3);
        //assert_eq!(ngrams.next(), Some("ζoo"));
        //assert_eq!(ngrams.next(), Some("oob"));
        //assert_eq!(ngrams.next(), Some("obα"));
        //assert_eq!(ngrams.next(), Some("bαr"));
        //assert_eq!(ngrams.next(), Some("αr "));
        //assert_eq!(ngrams.next(), Some("r 💖"));
        //assert_eq!(ngrams.next(), None);
    //}
}
