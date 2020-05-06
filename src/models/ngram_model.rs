use itertools::Itertools;
use models::errors::NGramModelError;
use std::collections::HashMap; // for cartesian_product

pub struct NGramGenerator {
    pub unigrams: Vec<String>,
}

impl NGramGenerator {
    pub fn generate(&self, ngram_length: usize) -> Vec<String> {
        self.recursion(self.unigrams.clone(), ngram_length - 1)
    }

    fn recursion(&self, ngrams: Vec<String>, index: usize) -> Vec<String> {
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

/// Hold mapping of ngrams to the related occurency counts
///
/// Occurency counts are stored as `f64` float to allow smoothing.
///
/// # Fields
///
/// * `model` - mapping of ngrams to occurence counts
pub struct NGramModel {
    model: HashMap<String, f64>, // float because of smoothing
}

impl NGramModel {
    pub fn from_ngrams(ngrams: &Vec<String>) -> Result<NGramModel, NGramModelError> {
        let model: HashMap<String, f64> = ngrams
            .into_iter()
            .map(move |ngram| (ngram.clone(), 0.0))
            .collect::<HashMap<String, f64>>();
        Ok(NGramModel { model })
    }

    pub fn add_ngrams(&mut self, ngrams: &Vec<String>) -> Result<(), NGramModelError> {
        for ngram in ngrams {
            self.add_ngram(&ngram[..])?;
        }
        Ok(())
    }

    pub fn add_ngram(&mut self, ngram: &str) -> Result<(), NGramModelError> {
        match self.model.get_mut(ngram) {
            Some(count) => *count += 1.0,
            None => return Err(NGramModelError::new(&format!("Unknow key: {}", ngram))),
        };
        Ok(())
    }

    /// Provide occurence count for ngram
    pub fn get_ngram_count(&self, ngram: &str) -> Option<&f64> {
        self.model.get(ngram)
    }

    /// Provide sum of all ngram counts
    pub fn get_total_ngram_count(&self) -> f64 {
        self.model.iter().map(|(_, count)| count).sum()
    }

    /// Provide number of distinct ngrams in vocabulary
    pub fn get_vocabulary_size(&self) -> usize {
        self.model.iter().count()
    }

    pub fn get_mut_model(&mut self) -> &mut HashMap<String, f64> {
        &mut self.model
    }

    /// Provide number of ngrams seen as least once in text
    pub fn get_seen_type_count(&self) -> usize {
        self.model.iter().filter(|(_, count)| *count > &0.0).count()
    }

    /// Provide number of ngrams not seen in text
    pub fn get_unseen_type_count(&self) -> usize {
        self.model
            .iter()
            .filter(|(_, count)| *count == &0.0)
            .count()
    }

    /// Provide iterator over ngram model's (ngram, count) pairs
    pub fn iter(&self) -> impl Iterator<Item = (&String, &f64)> {
        self.model.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use models::sigma::{Sigma, SigmaType};
    use models::NGramExt;
    use models::text_model::TextModel;

    #[test]
    fn test_ngram_model1() {
        let set_marker: Option<u8> = Some(35);
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::Test);
        let text = String::from("aabcbaa");
        let ngram_length: usize = 1;
        let mut text_model = TextModel::new(ngram_length, &sigma).unwrap();
        text_model.add(&text[..]);
        let ngrams = text_model.iter_ngrams(ngram_length).collect::<Vec<String>>();
        let mut ngram_model = NGramModel::from_ngrams(&sigma.string_ngrams(ngram_length)).unwrap();
        ngram_model.add_ngrams(&ngrams).unwrap();
        assert_eq!(&4.0, ngram_model.get_ngram_count("a").unwrap());
        assert_eq!(&2.0, ngram_model.get_ngram_count("b").unwrap());
        assert_eq!(&1.0, ngram_model.get_ngram_count("c").unwrap());
        assert_eq!(&2.0, ngram_model.get_ngram_count("#").unwrap());
    }

    #[test]
    fn test_ngram_model2() {
        let set_marker: Option<u8> = Some(35);
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::Test);
        let ngram_length: usize = 1;
        let text = String::from("aabcbaa");
        let mut text_model = TextModel::new(ngram_length, &sigma).unwrap();
        text_model.add(&text[..]);
        let ngrams = text_model.iter_ngrams(ngram_length).collect::<Vec<String>>();
        let mut ngram_model = NGramModel::from_ngrams(&sigma.string_ngrams(ngram_length)).unwrap();
        ngram_model.add_ngrams(&ngrams).unwrap();
        assert_eq!(&4.0, ngram_model.get_ngram_count("a").unwrap());
        assert_eq!(&2.0, ngram_model.get_ngram_count("b").unwrap());
        assert_eq!(&1.0, ngram_model.get_ngram_count("c").unwrap());
        assert_eq!(&2.0, ngram_model.get_ngram_count("#").unwrap());
    }

    #[test]
    fn test_ngram_model3() {
        let set_marker: Option<u8> = None;
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::Test);
        let text = String::from("aabcbaa");
        let ngram_length: usize = 2;
        let mut text_model = TextModel::new(ngram_length, &sigma).unwrap();
        text_model.add(&text[..]);
        let ngrams = text_model.iter_ngrams(ngram_length).collect::<Vec<String>>();
        let mut ngram_model = NGramModel::from_ngrams(&sigma.string_ngrams(ngram_length)).unwrap();
        ngram_model.add_ngrams(&ngrams).unwrap();
        assert_eq!(&2.0, ngram_model.get_ngram_count("aa").unwrap());
        assert_eq!(&1.0, ngram_model.get_ngram_count("ab").unwrap());
        assert_eq!(&0.0, ngram_model.get_ngram_count("bb").unwrap());
        assert_eq!(&1.0, ngram_model.get_ngram_count("ba").unwrap());
        assert_eq!(&1.0, ngram_model.get_ngram_count("cb").unwrap());
        assert_eq!(&0.0, ngram_model.get_ngram_count("ca").unwrap());
        assert_eq!(&1.0, ngram_model.get_ngram_count("bc").unwrap());
        assert_eq!(&0.0, ngram_model.get_ngram_count("ac").unwrap());
    }

    #[test]
    fn test_ngram_model4() {
        let set_marker: Option<u8> = Some(35);
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::Test);
        let text = String::from("aabcbaa");
        let ngram_length: usize = 2;
        let mut text_model = TextModel::new(ngram_length, &sigma).unwrap();
        text_model.add(&text[..]);
        let ngrams = text_model.iter_ngrams(ngram_length).collect::<Vec<String>>();
        let mut ngram_model = NGramModel::from_ngrams(&sigma.string_ngrams(ngram_length)).unwrap();
        ngram_model.add_ngrams(&ngrams).unwrap();
        assert_eq!(&2.0, ngram_model.get_ngram_count("aa").unwrap());
        assert_eq!(&1.0, ngram_model.get_ngram_count("ab").unwrap());
        assert_eq!(&0.0, ngram_model.get_ngram_count("bb").unwrap());
        assert_eq!(&1.0, ngram_model.get_ngram_count("ba").unwrap());
        assert_eq!(&0.0, ngram_model.get_ngram_count("ca").unwrap());
        assert_eq!(&1.0, ngram_model.get_ngram_count("cb").unwrap());
        assert_eq!(&0.0, ngram_model.get_ngram_count("ac").unwrap());
        assert_eq!(&1.0, ngram_model.get_ngram_count("bc").unwrap());
        assert_eq!(&0.0, ngram_model.get_ngram_count("cc").unwrap());
        assert_eq!(&2.0, ngram_model.get_ngram_count("##").unwrap());
        assert_eq!(&1.0, ngram_model.get_ngram_count("#a").unwrap());
        assert_eq!(&0.0, ngram_model.get_ngram_count("#b").unwrap());
        assert_eq!(&0.0, ngram_model.get_ngram_count("b#").unwrap());
        assert_eq!(&1.0, ngram_model.get_ngram_count("a#").unwrap());
        assert_eq!(&0.0, ngram_model.get_ngram_count("c#").unwrap());
        assert_eq!(&0.0, ngram_model.get_ngram_count("#c").unwrap());
    }

    #[test]
    fn test_ngram_model5() {
        let set_marker: Option<u8> = None;
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::Test);
        let text = String::from("aabbba");
        let ngram_length: usize = 2;
        let mut text_model = TextModel::new(ngram_length, &sigma).unwrap();
        text_model.add(&text[..]);
        let ngrams = text_model.iter_ngrams(ngram_length).collect::<Vec<String>>();
        let mut ngram_model = NGramModel::from_ngrams(&sigma.string_ngrams(ngram_length)).unwrap();
        ngram_model.add_ngrams(&ngrams).unwrap();
        let count: f64 = ngram_model.get_total_ngram_count();
        assert_eq!(5.0, count);
    }

    #[test]
    fn test_ngram_model6() {
        let set_marker: Option<u8> = None;
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::Test);
        let text = String::from("aabbbac");
        let ngram_length: usize = 2;
        let mut text_model = TextModel::new(ngram_length, &sigma).unwrap();
        text_model.add(&text[..]);
        let ngrams = text_model.iter_ngrams(ngram_length).collect::<Vec<String>>();
        let mut ngram_model = NGramModel::from_ngrams(&sigma.string_ngrams(ngram_length)).unwrap();
        ngram_model.add_ngrams(&ngrams).unwrap();
        let count: usize = ngram_model.get_vocabulary_size();
        assert_eq!(9, count);
    }

    #[test]
    fn test_ngram_model7() {
        let set_marker: Option<u8> = None;
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::Test);
        let text = String::from("abbba");
        let ngram_length: usize = 2;
        let mut text_model = TextModel::new(ngram_length, &sigma).unwrap();
        text_model.add(&text[..]);
        let ngrams = text_model.iter_ngrams(ngram_length).collect::<Vec<String>>();
        let mut ngram_model = NGramModel::from_ngrams(&sigma.string_ngrams(ngram_length)).unwrap();
        ngram_model.add_ngrams(&ngrams).unwrap();
        let count: usize = ngram_model.get_seen_type_count();
        assert_eq!(3, count);
    }

    #[test]
    fn test_ngram_model8() {
        let set_marker: Option<u8> = None;
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::Test);
        let text = String::from("abbba");
        let ngram_length: usize = 2;
        let mut text_model = TextModel::new(ngram_length, &sigma).unwrap();
        text_model.add(&text[..]);
        let ngrams = text_model.iter_ngrams(ngram_length).collect::<Vec<String>>();
        let mut ngram_model = NGramModel::from_ngrams(&sigma.string_ngrams(ngram_length)).unwrap();
        ngram_model.add_ngrams(&ngrams).unwrap();
        let count: usize = ngram_model.get_unseen_type_count();
        assert_eq!(6, count);
    }
}
