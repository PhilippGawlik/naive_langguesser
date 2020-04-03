use text_processing::errors::TextError;
use ngram::NgramExt;

pub mod errors;


pub fn preprocess_text(sigma: &str, raw_str: &str, set_marker: Option<usize>) -> Result<String, TextError> {
    let normalised_text: String = normalise_text(&sigma[..], &raw_str[..])?;
    match set_marker {
        Some(marker_length) => {
            Ok(add_marker(&normalised_text[..], marker_length)?)
        }
        None => Ok(normalised_text)
    }
}


fn normalise_text(sigma: &str, raw_text: &str) -> Result<String,TextError> {
    Ok(raw_text
        .chars()
        .filter_map(|s: char| in_sigma(sigma, s))
        .collect::<String>())
}


fn in_sigma(sigma: &str, symbol: char) -> Option<char> {
    match sigma.contains(symbol) {
        true => Some(symbol),
        false => None
    }
}


fn add_marker(text: &str, marker_length: usize) -> Result<String,TextError> {
    let marker_string: String = match marker_length {
        1 => "#".to_string(),
        2 => "##".to_string(),
        3 => "###".to_string(),
        _ => panic!("Caught marker length of size {}", marker_length)
    };
    Ok(format!("{}{}{}", marker_string, text.to_string(), marker_string))
}


pub fn get_ngrams(text: &str, n: usize) -> Result<Vec<String>, TextError> {
    Ok(text
        .char_ngrams(n)
        .map(|c| c.to_string())
        .collect::<Vec<String>>())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_normalise_text() {
        let sigma = "abcdefghijklmnopqrstuvwxyz0123456789".to_string();
        let raw_text = String::from("ß;b--a💖\t");
        assert_eq!(String::from("ba"), normalise_text(&sigma[..], &raw_text[..]).unwrap());
    }

    #[test]
    fn test_ngram_getter() {
        let text: String = "abc".to_string();
        let unigram_gold = vec!["a", "b", "c"];
        assert_eq!(unigram_gold, get_ngrams(&text[..], 1).unwrap());
        let bigram_gold = vec!["ab", "bc"];
        assert_eq!(bigram_gold, get_ngrams(&text[..], 2).unwrap());
        let trigram_gold = vec!["abc"];
        assert_eq!(trigram_gold, get_ngrams(&text[..], 3).unwrap());
    }
}
