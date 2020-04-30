use models::sigma::{Sigma, get_ngrams};
use std::collections::HashSet;
use text_processing::errors::TextError;

pub struct TextModel {
    set_confix: Option<String>,
    sigma: HashSet<u8>,
    text: Vec<u8>,
}

impl TextModel {
    pub fn new(
        ngram_length: usize,
        set_marker: Option<u8>,
        sigma: &Sigma,
    ) -> Result<TextModel, TextError> {
        let sigma_as_bytes: HashSet<u8> = match set_marker {
            Some(marker_byte) => {
                let mut bytes = sigma.as_bytes();
                bytes.insert(marker_byte);
                bytes
            }
            None => sigma.as_bytes(),
        };
        let set_confix: Option<String> = match set_marker {
            Some(marker_byte) => Some(TextModel::get_confix(marker_byte, ngram_length)?),
            None => None,
        };
        Ok(TextModel {
            set_confix,
            sigma: sigma_as_bytes,
            text: Vec::new(),
        })
    }

    fn get_confix(marker_byte: u8, ngram_length: usize) -> Result<String, TextError> {
        let bytes: Vec<u8> = (0..ngram_length)
            .into_iter()
            .map(|_| marker_byte)
            .collect::<Vec<u8>>();
        match String::from_utf8(bytes) {
            Ok(confix) => Ok(confix),
            Err(err) => Err(TextError::new(
                &format!("Error while generating confix: {}", err)[..],
            )),
        }
    }

    pub fn add(&mut self, text: &str) {
        let extension = text
            .as_bytes()
            .into_iter()
            .filter_map(|b| self.in_sigma(b))
            .collect::<Vec<u8>>();
        self.text.extend(extension);
    }

    fn in_sigma(&self, byte: &u8) -> Option<u8> {
        match self.sigma.contains(byte) {
            true => Some(byte.clone()),
            false => None,
        }
    }

    pub fn get_sigma(&self) -> &HashSet<u8> {
        &self.sigma
    }

    pub fn get_text(&self) -> String {
        let text: String = match String::from_utf8(self.text.clone()) {
            Ok(string) => string,
            Err(err) => panic!("Error while casting internal vec<u8> to String: {}", err),
        };
        match &self.set_confix {
            Some(confix) => format!("{}{}{}", confix.clone(), text, confix),
            None => text,
        }
    }

    pub fn get_ngrams(&self, ngram_length: usize) -> Vec<String> {
        let text: String = self.get_text();
        match ngram_length {
            1 => get_ngrams(&text[..], 1),
            2 => get_ngrams(&text[..], 2),
            3 => get_ngrams(&text[..], 3),
            _ => panic!(
                "NGram generator for length {} not implemented",
                ngram_length
            ),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_alphanum_text_model1() {
        let raw_text = String::from("abc\t");
        let sigma: Sigma = Sigma::AlphaNum;
        let set_marker: Option<u8> = None;
        let ngram_length: usize = 1;
        let mut text_model = TextModel::new(ngram_length, set_marker, &sigma).unwrap();
        text_model.add(&raw_text[..]);
        let text = text_model.get_text();
        assert_eq!("abc", &text[..]);
    }

    #[test]
    fn test_alphanum_text_model2() {
        let raw_text = String::from("abc\t");
        let sigma: Sigma = Sigma::AlphaNum;
        let set_marker: Option<u8> = Some(35);
        let ngram_length: usize = 3;
        let mut text_model = TextModel::new(ngram_length, set_marker, &sigma).unwrap();
        text_model.add(&raw_text[..]);
        let text = text_model.get_text();
        assert_eq!("###abc###", &text[..]);
    }

    #[test]
    fn test_ascii_text_model() {
        let input = String::from("abc💖");
        let sigma: Sigma = Sigma::Ascii;
        let set_marker: Option<u8> = None;
        let ngram_length: usize = 3;
        let mut text_model = TextModel::new(ngram_length, set_marker, &sigma).unwrap();
        text_model.add(&input[..]);
        let text = text_model.get_text();
        assert_eq!("abc", &text[..]);
    }
}
