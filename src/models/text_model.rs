use models::sigma::Sigma;
use text_processing::errors::TextError;
use std::str;

pub struct TextModel {
    set_confix: Option<String>,
    sigma: Sigma,
    text: Vec<u8>,
}

impl TextModel {
    pub fn new(
        ngram_length: usize,
        sigma_ref: &Sigma,
    ) -> Result<TextModel, TextError> {
        let sigma: Sigma = sigma_ref.clone();
        let set_confix: Option<String> = match sigma.set_marker {
            Some(marker_byte) => Some(TextModel::get_confix(marker_byte, ngram_length)?),
            None => None,
        };
        Ok(TextModel {
            set_confix,
            sigma,
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
            .filter_map(|b| self.sigma.contains(b))
            .collect::<Vec<u8>>();
        self.text.extend(extension);
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
}
