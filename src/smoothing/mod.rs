use models::SimpleModel;
use smoothing::errors::SmoothingError;
use std::collections::HashMap;
use utils::get_probability;
// for testing

pub mod errors;

pub enum SmoothingType {
    NoSmoothing,
    AddOneSmoothing,
    WittenBellSmoothing,
}

pub fn smoothing(
    count_model: &SimpleModel,
    type_: SmoothingType,
) -> Result<HashMap<String, f32>, SmoothingError> {
    match type_ {
        SmoothingType::NoSmoothing => no_smoothing(count_model),
        SmoothingType::AddOneSmoothing => add_one_smoothing(count_model),
        SmoothingType::WittenBellSmoothing => witten_bell_smoothing_flat(count_model),
    }
}

pub fn no_smoothing(count_model: &SimpleModel) -> Result<HashMap<String, f32>, SmoothingError> {
    let total_ngram_counts: i32 = count_model.get_total_ngram_count();
    Ok(count_model
        .model
        .iter()
        .map(|(ngram, c)| {
            let prob: f32 = get_probability(*c as f32, total_ngram_counts as f32).unwrap();
            (ngram.to_string(), prob)
        })
        .collect::<HashMap<String, f32>>())
}

pub fn add_one_smoothing(
    count_model: &SimpleModel,
) -> Result<HashMap<String, f32>, SmoothingError> {
    let total_ngram_counts: f32 = count_model.get_total_ngram_count() as f32;
    let vocabulary_size: f32 = count_model.get_vocabulary_size() as f32;
    let normalization_term: f32 = total_ngram_counts + vocabulary_size;
    Ok(count_model
        .model
        .iter()
        .map(|(ngram, c)| {
            let add_one: f32 = (c + 1) as f32;
            let prob: f32 = get_probability(add_one, normalization_term).unwrap();
            (ngram.to_string(), prob)
        })
        .collect::<HashMap<String, f32>>())
}

pub fn witten_bell_smoothing_flat(
    count_model: &SimpleModel,
) -> Result<HashMap<String, f32>, SmoothingError> {
    let total_ngram_count: f32 = count_model.get_total_ngram_count() as f32;
    let seen_count: f32 = count_model.get_seen_type_count() as f32;
    let unseen_count: f32 = count_model.get_unseen_type_count() as f32;
    let normalization_term: f32 = total_ngram_count + seen_count;
    let unseen_prob: f32 = seen_count / (unseen_count * normalization_term);
    Ok(count_model
        .model
        .iter()
        .map(|(ngram, c)| {
            if *c == 0 {
                (ngram.to_string(), unseen_prob)
            } else {
                (ngram.to_string(), (*c as f32 / normalization_term))
            }
        })
        .collect::<HashMap<String, f32>>())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_smoothing() {
        let sigma = String::from("ab");
        let feature_length = 2;
        let mut simple_model = SimpleModel::new(&sigma, feature_length).unwrap();
        let content = "aabcbba";
        simple_model.add_content(&content);
        let probs = smoothing(&simple_model, SmoothingType::NoSmoothing).unwrap();
        assert_eq!(&0.25, probs.get(&String::from("aa")).unwrap());
        assert_eq!(&0.25, probs.get(&String::from("ab")).unwrap());
        assert_eq!(&0.25, probs.get(&String::from("bb")).unwrap());
        assert_eq!(&0.25, probs.get(&String::from("ba")).unwrap());
    }

    #[test]
    fn test_add_one_smoothing() {
        let sigma = String::from("abc");
        let feature_length = 2;
        let mut simple_model = SimpleModel::new(&sigma, feature_length).unwrap();
        let content = "aaacbba";
        simple_model.add_content(&content);
        let probs = smoothing(&simple_model, SmoothingType::AddOneSmoothing).unwrap();
        assert_eq!(&(1.2 / 6.0), probs.get(&String::from("aa")).unwrap());
        assert_eq!(&(0.4 / 6.0), probs.get(&String::from("ab")).unwrap());
        assert_eq!(&(0.8 / 6.0), probs.get(&String::from("bb")).unwrap());
        assert_eq!(&(0.8 / 6.0), probs.get(&String::from("ba")).unwrap());
        assert_eq!(&(0.8 / 6.0), probs.get(&String::from("ac")).unwrap());
        assert_eq!(&(0.4 / 6.0), probs.get(&String::from("ca")).unwrap());
        assert_eq!(&(0.4 / 6.0), probs.get(&String::from("bc")).unwrap());
        assert_eq!(&(0.8 / 6.0), probs.get(&String::from("cb")).unwrap());
        assert_eq!(&(0.4 / 6.0), probs.get(&String::from("cc")).unwrap());
    }

    #[test]
    fn test_witten_bell_smoothing_flat() {
        let sigma = String::from("abc");
        let feature_length = 2;
        let mut simple_model = SimpleModel::new(&sigma, feature_length).unwrap();
        let content = "aaacbba";
        simple_model.add_content(&content);
        let probs = smoothing(&simple_model, SmoothingType::WittenBellSmoothing).unwrap();
        assert_eq!(&0.181818182, probs.get(&String::from("aa")).unwrap());
        assert_eq!(&0.11363637, probs.get(&String::from("ab")).unwrap());
        assert_eq!(&0.090909091, probs.get(&String::from("bb")).unwrap());
        assert_eq!(&0.090909091, probs.get(&String::from("ba")).unwrap());
        assert_eq!(&0.090909091, probs.get(&String::from("ac")).unwrap());
        assert_eq!(&0.11363637, probs.get(&String::from("ca")).unwrap());
        assert_eq!(&0.11363637, probs.get(&String::from("bc")).unwrap());
        assert_eq!(&0.090909091, probs.get(&String::from("cb")).unwrap());
        assert_eq!(&0.11363637, probs.get(&String::from("cc")).unwrap());
    }
}
