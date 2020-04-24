use models::ngram_model::NGramModel;
use smoothing::errors::SmoothingError;
use std::collections::HashMap;

pub mod errors;

/// Present types of smoothing
///
/// For further information see:
///
/// Speech and Language Processing
/// Daniel Jurafsky / James H. Martin
/// page 206: Smoothing
/// ISBN 0-13-095069-6
///
/// # NoSmoothing
///
/// Don't perform smoothing/ leave counts unchanged
///
/// # AddOneSmoothing
///
/// Add one to every ngram to deal with unseen ngrams.
/// Adjust population accordingly.
///
/// # WittenBellSmoothing
///
/// Use the count of ngrams seen once to estimate the count of ngrams not seen.
///
pub enum SmoothingType {
    NoSmoothing,
    AddOneSmoothing,
    WittenBellSmoothing,
}

/// Performs a redistribution of ngram counts to fill unseen ngrams
pub fn smoothing(
    ngram_model: &mut NGramModel,
    type_: &SmoothingType,
) -> Result<(), SmoothingError> {
    match type_ {
        SmoothingType::NoSmoothing => Ok(()),
        SmoothingType::AddOneSmoothing => add_one_to_ngram_model(ngram_model),
        SmoothingType::WittenBellSmoothing => witten_bell_on_ngram_model(ngram_model),
    }
}

/// Add one to every ngram to deal with unseen ngrams
fn add_one_to_ngram_model(ngram_model: &mut NGramModel) -> Result<(), SmoothingError> {
    let total: f64 = ngram_model.get_total_ngram_count();
    let vocabulary_size: f64 = ngram_model.get_vocabulary_size() as f64;
    let model: &mut HashMap<String, f64> = ngram_model.get_mut_model();
    add_one_to_hashmap(model, total, vocabulary_size)?;
    Ok(())
}

fn add_one_to_hashmap(
    model: &mut HashMap<String, f64>,
    total: f64,
    vocabulary_size: f64
) -> Result<(), SmoothingError> {
    let normalization_term: f64 = total / (total + vocabulary_size);
    for (_ngram, count) in model.iter_mut() {
        *count = (*count + 1.0) * normalization_term;
    };
    Ok(())
}

/// Use count ngrams seen once to estimate count of unseen ngrams
fn witten_bell_on_ngram_model(ngram_model: &mut NGramModel) -> Result<(), SmoothingError> {
    let total: f64 = ngram_model.get_total_ngram_count();
    let seen: f64 = ngram_model.get_seen_type_count() as f64;
    let unseen: f64 = ngram_model.get_unseen_type_count() as f64;
    let model: &mut HashMap<String, f64> = ngram_model.get_mut_model();
    witten_bell_on_hashmap(model, total, seen, unseen)?;
    Ok(())
}

fn witten_bell_on_hashmap(
    model: &mut HashMap<String, f64>,
    total: f64,
    seen: f64,
    unseen: f64
) -> Result<(), SmoothingError> {
    if seen == 0.0 || unseen == 0.0 {
        return Ok(())
    };
    let normalization_term: f64 =  total / (total + seen);
    let smoothed_unseen: f64 = (seen/ unseen) * normalization_term;
    for (_ngram, count) in model.iter_mut() {
        if *count > 0.0 {
            *count = *count * normalization_term;
        } else {
            *count = smoothed_unseen;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_witten_bell_smoothing1() {
        let mut model: HashMap<String, f64> = HashMap::new();
        model.insert("a".to_string(), 2.0);
        model.insert("b".to_string(), 2.0);
        model.insert("c".to_string(), 0.0);

        let total: f64 = 4.0;
        let seen: f64 = 2.0;
        let unseen: f64 = 1.0;

        witten_bell_on_hashmap(&mut model, total, seen, unseen).unwrap();
        assert_eq!(
            &1.3333333333333333,
            model.get("a").unwrap()
        );
        assert_eq!(
            &1.3333333333333333,
            model.get("b").unwrap()
        );
        assert_eq!(
            &1.3333333333333333,
            model.get("c").unwrap()
        );
    }

    #[test]
    fn test_witten_bell_smoothing2() {
        let mut model: HashMap<String, f64> = HashMap::new();
        model.insert("a".to_string(), 1.0);
        model.insert("b".to_string(), 1.0);
        model.insert("c".to_string(), 1.0);

        let total: f64 = 3.0;
        let seen: f64 = 3.0;
        let unseen: f64 = 0.0;

        witten_bell_on_hashmap(&mut model, total, seen, unseen).unwrap();
        assert_eq!(
            &1.0000000000000000,
            model.get("a").unwrap()
        );
        assert_eq!(
            &1.0000000000000000,
            model.get("b").unwrap()
        );
        assert_eq!(
            &1.0000000000000000,
            model.get("c").unwrap()
        );
    }

    #[test]
    fn test_witten_bell_smoothing3() {
        let mut model: HashMap<String, f64> = HashMap::new();
        model.insert("a".to_string(), 0.0);
        model.insert("b".to_string(), 0.0);
        model.insert("c".to_string(), 0.0);

        let total: f64 = 0.0;
        let seen: f64 = 0.0;
        let unseen: f64 = 3.0;

        witten_bell_on_hashmap(&mut model, total, seen, unseen).unwrap();
        assert_eq!(
            &0.0000000000000000,
            model.get("a").unwrap()
        );
        assert_eq!(
            &0.0000000000000000,
            model.get("b").unwrap()
        );
        assert_eq!(
            &0.0000000000000000,
            model.get("c").unwrap()
        );
    }

    #[test]
    fn test_add_one_smoothing1() {
        let mut model: HashMap<String, f64> = HashMap::new();
        model.insert("a".to_string(), 2.0);
        model.insert("b".to_string(), 2.0);
        model.insert("c".to_string(), 0.0);
        let total: f64 = 4.0;
        let vocabulary_size: f64 = 3.0;
        add_one_to_hashmap(&mut model, total, vocabulary_size).unwrap();
        assert_eq!(
            &1.7142857142857142,
            model.get("a").unwrap()
        );
        assert_eq!(
            &1.7142857142857142,
            model.get("b").unwrap()
        );
        assert_eq!(
            &0.5714285714285714,
            model.get("c").unwrap()
        );
    }

    #[test]
    fn test_add_one_smoothing2() {
        let mut model: HashMap<String, f64> = HashMap::new();
        model.insert("a".to_string(), 0.0);
        model.insert("b".to_string(), 0.0);
        model.insert("c".to_string(), 0.0);
        let total: f64 = 0.0;
        let vocabulary_size: f64 = 3.0;
        add_one_to_hashmap(&mut model, total, vocabulary_size).unwrap();
        assert_eq!(
            &0.0000000000000000,
            model.get("a").unwrap()
        );
        assert_eq!(
            &0.0000000000000000,
            model.get("b").unwrap()
        );
        assert_eq!(
            &0.0000000000000000,
            model.get("c").unwrap()
        );
    }

    #[test]
    fn test_add_one_smoothing3() {
        let mut model: HashMap<String, f64> = HashMap::new();
        model.insert("a".to_string(), 1.0);
        model.insert("b".to_string(), 1.0);
        model.insert("c".to_string(), 1.0);
        let total: f64 = 3.0;
        let vocabulary_size: f64 = 3.0;
        add_one_to_hashmap(&mut model, total, vocabulary_size).unwrap();
        assert_eq!(
            &1.0000000000000000,
            model.get("a").unwrap()
        );
        assert_eq!(
            &1.0000000000000000,
            model.get("b").unwrap()
        );
        assert_eq!(
            &1.0000000000000000,
            model.get("c").unwrap()
        );
    }
}
