use models::sigma::{Sigma, Symbol};
use std::iter::FromIterator;
use std::str;
use text_processing::errors::TextError;

pub struct TextModel {
    set_confix: Option<Vec<Symbol>>,
    sigma: Sigma,
    text: Vec<Symbol>,
}

impl TextModel {
    pub fn new(ngram_length: usize, sigma_ref: &Sigma) -> Result<TextModel, TextError> {
        let set_confix: Option<Vec<Symbol>> = match &sigma_ref.set_marker {
            Some(marker_byte) => Some(TextModel::get_confix(marker_byte, ngram_length)),
            None => None,
        };
        let sigma: Sigma = sigma_ref.clone();
        Ok(TextModel {
            set_confix,
            sigma,
            text: Vec::new(),
        })
    }

    fn get_confix(marker_byte: &Symbol, ngram_length: usize) -> Vec<Symbol> {
        (0..ngram_length)
            .into_iter()
            .map(|_| marker_byte.clone())
            .collect::<Vec<Symbol>>()
    }

    fn confix_as_string(&self) -> String {
        match &self.set_confix {
            Some(confix) => String::from_iter(
                confix
                    .iter()
                    .map(|symbol| symbol.as_str())
                    .collect::<Vec<&str>>(),
            ),
            None => String::from(""),
        }
    }

    pub fn add(&mut self, text: &str) {
        let extension = text
            .as_bytes()
            .into_iter()
            .map(|byte| Symbol::from_u8(*byte)) //Todo add Jean approach
            .filter_map(|symbol| self.sigma.contains(symbol))
            .collect::<Vec<Symbol>>();
        self.text.extend(extension);
    }

    pub fn get_text(&self) -> String {
        let text: String = String::from_iter(
            self.text
                .iter()
                .map(|symbol| symbol.as_str())
                .collect::<Vec<&str>>(),
        );
        match &self.set_confix {
            Some(_) => format!(
                "{}{}{}",
                self.confix_as_string(),
                text,
                self.confix_as_string()
            ),
            None => text,
        }
    }

    pub fn get_symbols(&self) -> Vec<Symbol> {
        match &self.set_confix {
            Some(confix) => {
                let mut text_buffer: Vec<Symbol> = confix.clone();
                text_buffer.extend(self.text.clone());
                text_buffer.extend(confix.clone());
                text_buffer
            }
            None => self.text.clone(),
        }
    }

    pub fn iter_ngrams(&self, ngram_length: usize) -> TextModelNGramIter {
        let symbols: Vec<Symbol> = self.get_symbols();
        let length: usize = symbols.len();
        TextModelNGramIter {
            idx: 0,
            ngram_length,
            text: symbols,
            text_length: length,
        }
    }
}

pub struct TextModelNGramIter {
    idx: usize,
    ngram_length: usize,
    text: Vec<Symbol>,
    text_length: usize,
}

impl Iterator for TextModelNGramIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let offset: usize = self.idx + self.ngram_length;
        if offset > self.text_length {
            return None;
        };
        let ngram_symbols = &self.text[self.idx..offset];
        self.idx += 1;
        Some(String::from_iter(
            ngram_symbols
                .iter()
                .map(|symbol| symbol.as_str())
                .collect::<Vec<&str>>(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use models::sigma::SigmaType;

    #[test]
    fn test_alphanum_text_model1() {
        let raw_text = String::from("abc\t");
        let set_marker: Option<u8> = None;
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::AlphaNum);
        let ngram_length: usize = 1;
        let mut text_model = TextModel::new(ngram_length, &sigma).unwrap();
        text_model.add(&raw_text[..]);
        let text = text_model.get_text();
        assert_eq!("abc", &text[..]);
    }

    #[test]
    fn test_alphanum_text_model2() {
        let raw_text = String::from("abc\t");
        let set_marker: Option<u8> = Some(35);
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::AlphaNum);
        let ngram_length: usize = 3;
        let mut text_model = TextModel::new(ngram_length, &sigma).unwrap();
        text_model.add(&raw_text[..]);
        let text = text_model.get_text();
        assert_eq!("###abc###", &text[..]);
    }

    #[test]
    fn test_ascii_text_model() {
        let input = String::from("abc💖");
        let set_marker: Option<u8> = None;
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::AlphaNum);
        let ngram_length: usize = 3;
        let mut text_model = TextModel::new(ngram_length, &sigma).unwrap();
        text_model.add(&input[..]);
        let text = text_model.get_text();
        assert_eq!("abc", &text[..]);
    }

    #[test]
    fn test_confix() {
        let set_marker: Option<u8> = Some(35);
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::AlphaNum);
        let text_model = TextModel::new(3, &sigma).unwrap();
        let confix: String = text_model.confix_as_string();
        assert_eq!(String::from("###"), confix);
    }

    #[test]
    fn test_text_model_ngram_iterator() {
        let input = String::from("abc💖");
        let set_marker: Option<u8> = None;
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::AlphaNum);
        let ngram_length: usize = 2;
        let mut text_model = TextModel::new(ngram_length, &sigma).unwrap();
        text_model.add(&input[..]);
        let mut iter = text_model.iter_ngrams(ngram_length);
        assert_eq!(Some(String::from("ab")), iter.next());
        assert_eq!(Some(String::from("bc")), iter.next());
        assert_eq!(None, iter.next());
    }
}
