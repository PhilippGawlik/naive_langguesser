use itertools::Itertools;
use ngram::NgramExt;
use std::collections::HashSet;
use std::sync::Arc;
use text_processing::errors::TextError;

pub mod errors;

/// Types alphabets for text processing
///
/// # Fields
///
/// `Test` - simple alphabet for tests
/// `Ascii` - Todo
/// `Unicode` - Todo
///
pub enum SigmaID {
    Test,
    Ascii,
    Unicode,
}

/// Exchange sigma_id for alphabet string and add optional marker symbol
pub fn get_sigma(sigma_id: SigmaID, set_marker: Option<&str>) -> String {
    match set_marker {
        Some(marker_symbol) => dissolve_sigma_id(sigma_id) + marker_symbol,
        None => dissolve_sigma_id(sigma_id),
    }
}

/// Exchange sigma_id for alphabet string
fn dissolve_sigma_id(sigma_id: SigmaID) -> String {
    match sigma_id {
        SigmaID::Test => "abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        SigmaID::Ascii => panic!("Alphabet ascii is not implemented"),
        SigmaID::Unicode => panic!("Alphabet unicode is not implemented"),
    }
}

/// Check, if symbol is in alphabet
fn in_sigma(sigma: &str, symbol: char) -> Option<char> {
    match sigma.contains(symbol) {
        true => Some(symbol),
        false => None,
    }
}

/// Normalise text
///
/// 1. step: filter symbols not in alphabet
/// 2. step: generate confix (marker of certain length)
/// 3. step: add confix around text
pub fn preprocess_text(
    sigma: &str,
    raw_str: &str,
    set_marker: Option<&str>,
    ngram_length: usize,
) -> Result<String, TextError> {
    let normalised: String = normalise_text(&sigma[..], &raw_str[..])?;
    match set_marker {
        Some(marker_symbol) => Ok(generate_and_add_confix(&normalised[..], marker_symbol, ngram_length)),
        None => Ok(normalised),
    }
}

fn normalise_text(sigma: &str, raw_text: &str) -> Result<String, TextError> {
    let normalised = raw_text
        .chars()
        .filter_map(|s: char| in_sigma(sigma, s))
        .collect::<String>();
    match normalised.is_empty() {
        true => Err(TextError::new("Text is empty")),
        false => Ok(normalised),
    }
}

fn generate_and_add_confix(normalised: &str, marker_symbol: &str, ngram_length: usize) -> String {
    let confix: String = generate_confix(marker_symbol, ngram_length);
    add_confix(&normalised[..], &confix[..])
}

fn generate_confix(marker_symbol: &str, ngram_length: usize) -> String {
    match ngram_length {
        1 => format!("{}", marker_symbol),
        2 => format!("{}{}", marker_symbol, marker_symbol),
        3 => format!("{}{}{}", marker_symbol, marker_symbol, marker_symbol),
        _ => panic!("Marker length of size {} not implemented", ngram_length),
    }
}

fn add_confix(text: &str, confix: &str) -> String {
    format!("{}{}{}", confix, text, confix)
}

pub fn get_ngrams(text: &str, n: usize) -> Vec<String> {
    text.char_ngrams(n)
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
}

/// Generate all ngrams of a certain length and alphabet
pub fn get_total_list_of_ngrams(sigma: &str, ngram_length: usize) -> Option<HashSet<String>> {
    match ngram_length {
        1 => Some(
            sigma
                .char_ngrams(1)
                .map(|c| c.to_string())
                .collect::<HashSet<String>>(),
        ),
        2 => {
            let sigma_shared = Arc::new(sigma);
            let shared1 = Arc::clone(&sigma_shared);
            let shared2 = Arc::clone(&sigma_shared);
            Some(
                shared1
                    .char_ngrams(1)
                    .cartesian_product(shared2.char_ngrams(1))
                    .map(|(a, b)| format!("{}{}", a, b))
                    .collect::<HashSet<String>>(),
            )
        }
        3 => {
            let sigma_shared = Arc::new(sigma);
            let shared1 = Arc::clone(&sigma_shared);
            let shared2 = Arc::clone(&sigma_shared);
            let shared3 = Arc::clone(&sigma_shared);
            Some(
                shared1
                    .char_ngrams(1)
                    .cartesian_product(
                        shared2
                            .char_ngrams(1)
                            .cartesian_product(shared3.char_ngrams(1)),
                    )
                    .map(|(a, (b, c))| format!("{}{}{}", a, b, c))
                    .collect::<HashSet<String>>(),
            )
        }
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_normalise_text() {
        let sigma = "abcdefghijklmnopqrstuvwxyz0123456789".to_string();
        let raw_text = String::from("ß;b--a💖\t");
        assert_eq!(
            String::from("ba"),
            normalise_text(&sigma[..], &raw_text[..]).unwrap()
        );
    }

    #[test]
    fn test_ngram_getter() {
        let text: String = "abc".to_string();
        let unigram_gold = vec!["a", "b", "c"];
        assert_eq!(unigram_gold, get_ngrams(&text[..], 1));
        let bigram_gold = vec!["ab", "bc"];
        assert_eq!(bigram_gold, get_ngrams(&text[..], 2));
        let trigram_gold = vec!["abc"];
        assert_eq!(trigram_gold, get_ngrams(&text[..], 3));
    }

    #[test]
    fn test_get_total_2grams() {
        let sigma = "a💖".to_string();
        let n_gram_length: usize = 2;
        let ngrams = get_total_list_of_ngrams(&sigma[..], n_gram_length).unwrap();
        assert_eq!(true, ngrams.contains(&String::from("aa")));
        assert_eq!(true, ngrams.contains(&String::from("a💖")));
        assert_eq!(true, ngrams.contains(&String::from("💖a")));
        assert_eq!(true, ngrams.contains(&String::from("💖💖")));
    }

    #[test]
    fn test_get_threegram_features() {
        let sigma = "ab".to_string();
        let n_gram_length: usize = 3;
        let ngrams = get_total_list_of_ngrams(&sigma[..], n_gram_length).unwrap();
        assert_eq!(true, ngrams.contains(&String::from("aaa")));
        assert_eq!(true, ngrams.contains(&String::from("aab")));
        assert_eq!(true, ngrams.contains(&String::from("aba")));
        assert_eq!(true, ngrams.contains(&String::from("abb")));
        assert_eq!(true, ngrams.contains(&String::from("baa")));
        assert_eq!(true, ngrams.contains(&String::from("bab")));
        assert_eq!(true, ngrams.contains(&String::from("bba")));
        assert_eq!(true, ngrams.contains(&String::from("bbb")));
    }
}
