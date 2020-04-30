use models::count_model::CountModel;
use models::errors::ProbabilityModelError;
use models::ngram_model::NGramModel;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;
// necessary import for .lines() method of BufReader
use std::io::prelude::*;


/// Mapping of ngrams on there occurence probability
///
/// # Fields
///
/// `model` -  mapping of ngrams on their occurence probability
/// `name` - probability model name (related to modeled language)
pub struct ProbabilityModel {
    model: HashMap<String, f64>,
    pub name: String,
}

impl ProbabilityModel {
    /// Init empty model with the given `name`
    pub fn from_name(name: &str) -> Result<ProbabilityModel, ProbabilityModelError> {
        let name: String = String::from(name);
        let model: HashMap<String, f64> = HashMap::new();
        return Ok(ProbabilityModel { name, model });
    }

    /// Load probability model from probability model dump
    pub fn from_file(path: &str) -> Result<ProbabilityModel, ProbabilityModelError> {
        let name = ProbabilityModel::parse_name_from_path(path)?;
        let mut model: HashMap<String, f64> = HashMap::new();
        let f = fs::File::open(path)?;
        let reader = io::BufReader::new(f);
        for line in reader.lines() {
            let mut split = line // looks like: abc\t0.123
                .as_ref() // 'line' needs to outlive 'split'
                .unwrap()
                .split("\t"); // split into: [abc, 0.123]
            let ngram = match split.next() {
                Some(ngram) => String::from(ngram),
                None => panic!("Illformed line in model: {}", &name[..]),
            };
            let probability: f64 = match split.next() {
                Some(raw) => raw.parse().unwrap(),
                None => panic!("Illformed prabability in model: {}", &name[..]),
            };
            model.insert(ngram, probability);
        }
        Ok(ProbabilityModel { name, model })
    }

    /// Add unigram probabilities from count model
    pub fn add_unigram_probabilities(
        &mut self,
        count_model: &CountModel,
    ) -> Result<(), ProbabilityModelError> {
        let unigram_counts: &NGramModel = match count_model.get_ngram_model(1) {
            Some(counts) => counts,
            None => {
                return Err(ProbabilityModelError::new(&format!(
                    "No unigram model found"
                )))
            }
        };
        self.calc_and_add_unigram_probabilities(unigram_counts)
    }

    /// Calculate unigram probabilities from unigram counts
    ///
    /// Calculation is done by dividing:
    /// |ngram| - count of unigram (e.g. |a| = 5)
    /// |total| - sum of all unigram counts (e.g. |total| = 100)
    ///
    ///  probability = |ngram| / |total|
    ///
    fn calc_and_add_unigram_probabilities(
        &mut self,
        unigram_model: &NGramModel,
    ) -> Result<(), ProbabilityModelError> {
        let total: f64 = unigram_model.get_total_ngram_count();
        for (ngram, count) in unigram_model.iter() {
            let prob: f64 = count / total;
            self.model.insert(ngram.to_string(), prob);
        }
        Ok(())
    }

    /// Add ngram probabilities from count model
    pub fn add_ngram_probabilities(
        &mut self,
        count_model: &CountModel,
    ) -> Result<(), ProbabilityModelError> {
        for (prefix_model, ngram_model) in count_model.iter_tuple() {
            self.calc_and_add_ngram_probabilites(prefix_model, ngram_model)?;
        }
        Ok(())
    }

    /// Calculate ngram probabilities from ngram counts
    ///
    /// Calculation is done by dividing:
    /// |ngram| - count of ngram (e.g. |abc| = 5)
    /// |prefix| - count of ngram prefix (e.g. |ab| = 7)
    ///
    ///  probability = |ngram| / |prefix|
    ///
    fn calc_and_add_ngram_probabilites(
        &mut self,
        prefix_model: &NGramModel,
        ngram_model: &NGramModel,
    ) -> Result<(), ProbabilityModelError> {
        for (ngram, count) in ngram_model.iter() {
            let prefix: String = ngram[..ngram.len() - 1].to_string();
            let denominator: f64 = match prefix_model.get_ngram_count(&prefix[..]) {
                Some(count) => *count,
                None => {
                    return Err(ProbabilityModelError::new(&format!(
                        "Ngram model doesn't know: {}",
                        prefix
                    )))
                }
            };
            self.model.insert(String::from(ngram), count / denominator);
        }
        Ok(())
    }

    pub fn get(&self, ngram: &str) -> Option<&f64> {
        self.model.get(ngram)
    }

    pub fn write_to_file(self, path: &str) -> Result<(), ProbabilityModelError> {
        let mut write_buf = String::new();
        for (ngram, prob) in self.model {
            write_buf.push_str(&format!("{}\t{}", ngram, prob));
            write_buf.push_str(&String::from("\n"));
        }
        fs::write(path, &write_buf)?;
        Ok(())
    }

    /// Parse model name from file name
    pub fn parse_name_from_path(path: &str) -> Result<String, ProbabilityModelError> {
        // only build regex once
        lazy_static! {
            static ref GET_NAME: Regex =
                Regex::new(r"\./data(?:/models)?/([a-zA-Z0-9]+)\.model").unwrap();
        };
        Ok(String::from(
            &GET_NAME
                .captures_iter(path)
                .next()
                .expect("Can't parse model name from path")[1],
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use smoothing::SmoothingType;
    use models::sigma::{Sigma, SigmaType};
    use TextModel;

    #[test]
    fn test_probability_model1() {
        let ngram_length: usize = 1;
        let set_marker: Option<u8> = None;
        let smoothing_type: SmoothingType = SmoothingType::NoSmoothing;
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::AlphaNum);
        let mut text_model = TextModel::new(ngram_length, &sigma).unwrap();
        let mut count_model = CountModel::from_sigma(&sigma, ngram_length).unwrap();
        let mut probability_model = ProbabilityModel::from_name("test").unwrap();
        let raw_text = String::from("aabcbaa\t");
        text_model.add(&raw_text[..]);
        count_model
            .count_ngrams_from_text_model(&text_model)
            .unwrap();
        count_model.smooth(&smoothing_type).unwrap();
        probability_model.add_unigram_probabilities(&count_model).unwrap();
        assert_eq!(&(4.0 / 7.0), probability_model.get("a").unwrap());
        assert_eq!(&(2.0 / 7.0), probability_model.get("b").unwrap());
        assert_eq!(&(1.0 / 7.0), probability_model.get("c").unwrap());
    }

    #[test]
    fn test_probability_model2() {
        let ngram_length: usize = 1;
        let set_marker: Option<u8> = Some(35);
        let smoothing_type: SmoothingType = SmoothingType::NoSmoothing;
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::AlphaNum);
        let mut text_model =
            TextModel::new(ngram_length, &sigma).unwrap();
        let mut count_model = CountModel::from_sigma(&sigma, ngram_length).unwrap();
        let mut probability_model = ProbabilityModel::from_name("test").unwrap();
        let raw_text = String::from("aabcbaa\t");
        text_model.add(&raw_text[..]);
        count_model
            .count_ngrams_from_text_model(&text_model)
            .unwrap();
        count_model.smooth(&smoothing_type).unwrap();
        probability_model.add_unigram_probabilities(&count_model).unwrap();
        assert_eq!(&(4.0 / 9.0), probability_model.get("a").unwrap());
        assert_eq!(&(2.0 / 9.0), probability_model.get("b").unwrap());
        assert_eq!(&(1.0 / 9.0), probability_model.get("c").unwrap());
    }

    #[test]
    fn test_probability_model3() {
        let raw_text = String::from("aabcbaa\t");
        let ngram_length: usize = 2;
        let set_marker: Option<u8> = Some(35);
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::AlphaNum);
        let smoothing_type: SmoothingType = SmoothingType::NoSmoothing;
        let mut text_model =
            TextModel::new(ngram_length, &sigma).unwrap();
        let mut count_model = CountModel::from_sigma(&sigma, ngram_length).unwrap();
        let mut probability_model = ProbabilityModel::from_name("test").unwrap();
        text_model.add(&raw_text[..]);
        count_model
            .count_ngrams_from_text_model(&text_model)
            .unwrap();
        count_model.smooth(&smoothing_type).unwrap();
        probability_model.add_unigram_probabilities(&count_model).unwrap();
        probability_model.add_ngram_probabilities(&count_model).unwrap();
        assert_eq!(&(2.0 / 4.0), probability_model.get("aa").unwrap());
        assert_eq!(&(1.0 / 4.0), probability_model.get("ab").unwrap());
        assert_eq!(&(0.0 / 4.0), probability_model.get("ac").unwrap());
        assert_eq!(&(1.0 / 4.0), probability_model.get("a#").unwrap());
        assert_eq!(&(0.0 / 2.0), probability_model.get("bb").unwrap());
        assert_eq!(&(1.0 / 2.0), probability_model.get("ba").unwrap());
        assert_eq!(&(1.0 / 2.0), probability_model.get("bc").unwrap());
        assert_eq!(&(0.0 / 2.0), probability_model.get("b#").unwrap());
        assert_eq!(&(0.0 / 1.0), probability_model.get("ca").unwrap());
        assert_eq!(&(1.0 / 1.0), probability_model.get("cb").unwrap());
        assert_eq!(&(0.0 / 1.0), probability_model.get("cc").unwrap());
        assert_eq!(&(0.0 / 1.0), probability_model.get("c#").unwrap());
        assert_eq!(&(2.0 / 4.0), probability_model.get("##").unwrap());
        assert_eq!(&(1.0 / 4.0), probability_model.get("#a").unwrap());
        assert_eq!(&(0.0 / 4.0), probability_model.get("#b").unwrap());
        assert_eq!(&(0.0 / 4.0), probability_model.get("#c").unwrap());
    }

    #[test]
    fn test_probability_model4() {
        let path = String::from("./data/test.model");
        let probability_model = ProbabilityModel::from_file(&path[..]).unwrap();
        assert_eq!(&0.13530510588511946, probability_model.get("a").unwrap());
        assert_eq!(&0.08394062078272607, probability_model.get("b").unwrap());
        assert_eq!(&0.13530510588511946, probability_model.get("c").unwrap());
        assert_eq!(&0.07047140931516639, probability_model.get("#").unwrap());
    }

    #[test]
    fn test_probability_model5() {
        let path = String::from("./data/models/test.model");
        assert_eq!(
            "test",
            ProbabilityModel::parse_name_from_path(&path[..]).unwrap()
        );
        let path2 = String::from("./data/test.model");
        assert_eq!(
            "test",
            ProbabilityModel::parse_name_from_path(&path2[..]).unwrap()
        );
    }
}
