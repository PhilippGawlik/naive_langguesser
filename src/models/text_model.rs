use text_processing::errors::TextError;
use text_processing::{get_ngrams, get_sigma, preprocess_text, SigmaID};


/// Hold text representation
///
/// # Fields
///
/// * `sigma` - text alphabet as `String` 
/// * `text` - text as `String` 
pub struct TextModel {
    sigma: String,
    text: String,
}

impl TextModel {
    /// Init `TextModel` from raw string
    /// 
    /// This includes converting `sigma_id` to sigma `String`
    /// and preprocessing the text.
    ///
    /// # Arguments
    ///
    /// * `raw_str` - unprocessed text as `String`
    /// * `set_marker` - `Option` deciding if marker is added to string and if so, what marker
    /// symbol to use
    /// * `ngram` - max ngram-length as `usize`
    pub fn from_raw(
        raw_str: &str,
        sigma_id: SigmaID,
        set_marker: Option<&str>,
        ngram_length: usize,
    ) -> Result<TextModel, TextError> {
        let sigma: String = get_sigma(sigma_id, set_marker);
        let text: String = preprocess_text(&sigma[..], &raw_str[..], set_marker, ngram_length)?;
        Ok(TextModel { sigma, text })
    }

    pub fn get_sigma(&self) -> &str {
        &self.sigma[..]
    }

    pub fn get_ngrams(&self, ngram_length: usize) -> Vec<String> {
        match ngram_length {
            1 => get_ngrams(&self.text[..], 1),
            2 => get_ngrams(&self.text[..], 2),
            3 => get_ngrams(&self.text[..], 3),
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
    fn test_text_model1() {
        let raw_text = String::from("abc\t");
        let sigma_id: SigmaID = SigmaID::Test;
        let set_marker: Option<&str> = None;
        let ngram_length: usize = 1;
        let text_model =
            TextModel::from_raw(&raw_text[..], sigma_id, set_marker, ngram_length).unwrap();
        let text = text_model.text;
        assert_eq!("abc", text);
    }

    #[test]
    fn test_text_model2() {
        let raw_text = String::from("abc\t");
        let sigma_id: SigmaID = SigmaID::Test;
        let set_marker: Option<&str> = Some("#");
        let ngram_length: usize = 3;
        let text_model =
            TextModel::from_raw(&raw_text[..], sigma_id, set_marker, ngram_length).unwrap();
        let text = text_model.text;
        assert_eq!("###abc###", text);
    }
}
