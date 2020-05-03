use models::errors::CountModelError;
use models::ngram_model::NGramModel;
use models::text_model::TextModel;
use smoothing::{smoothing, SmoothingType};
use std::collections::HashMap;
use models::NGramExt;
use models::sigma::Sigma;

/// Hold ngram occurence models of various length
pub struct CountModel {
    max_ngram_length: usize,
    ngram_models: HashMap<usize, NGramModel>,
}

impl CountModel {
    /// Init total `CountModel` from sigma (alphabet) and a max ngram length
    ///
    /// A `CountModel` is initialised for (0..=max_ngram_length) each from a total permutation of
    /// ngrams of the related length. The total permutation of ngrams of a certain length is build
    /// from sigma.
    ///
    /// # Arguments
    ///
    /// `sigma` -  relevant alphabet for ngrams
    /// `max_ngram_length` - max length of ngrams
    pub fn from_sigma(sigma: &Sigma, max_ngram_length: usize) -> Result<CountModel, CountModelError> {
        let mut ngram_models = HashMap::new();
        for ngram_length in 1..=max_ngram_length {
            let ngrams: Vec<String> = sigma.string_ngrams(ngram_length);
            let ngram_model = NGramModel::from_ngrams(&ngrams)?;
            ngram_models.insert(ngram_length, ngram_model);
        }
        Ok(CountModel {
            max_ngram_length,
            ngram_models,
        })
    }

    pub fn get_mut_ngram_model(&mut self, ngram_length: usize) -> Option<&mut NGramModel> {
        self.ngram_models.get_mut(&ngram_length)
    }

    pub fn get_ngram_model(&self, idx: usize) -> Option<&NGramModel> {
        self.ngram_models.get(&idx)
    }

    pub fn count_ngrams_from_text_model(
        &mut self,
        text_model: &TextModel,
    ) -> Result<(), CountModelError> {
        for idx in 1..=self.max_ngram_length {
            let ngrams: Vec<String> = text_model.string_ngrams(idx);
            self.count_ngrams(&ngrams, idx)?;
        }
        Ok(())
    }

    fn count_ngrams(
        &mut self,
        ngrams: &Vec<String>,
        ngram_length: usize,
    ) -> Result<(), CountModelError> {
        let ngram_model: &mut NGramModel = match self.get_mut_ngram_model(ngram_length) {
            Some(model) => model,
            None => {
                return Err(CountModelError::new(
                    &format!("Can't find count model for index: {}", ngram_length)[..],
                ))
            }
        };
        ngram_model.add_ngrams(&ngrams)?;
        Ok(())
    }

    /// Smooth collected ngram counts
    ///
    /// Smoothing performs an redistribution of ngram counts to fill unseen ngrams.
    ///
    /// # Arguments
    ///
    /// * `smoothing_type` - smoothing_type to be performed on counts
    pub fn smooth(&mut self, smoothing_type: &SmoothingType) -> Result<(), CountModelError> {
        for idx in 1..=self.max_ngram_length {
            let ngram_model: &mut NGramModel = match self.get_mut_ngram_model(idx) {
                Some(model) => model,
                None => {
                    return Err(CountModelError::new(
                        &format!("Can't find count model for index: {}", idx)[..],
                    ))
                }
            };
            smoothing(ngram_model, &smoothing_type)?;
        }
        Ok(())
    }

    /// Provide tuple iterator for count models
    ///
    /// Provide iterator about two count models of successive ngram length
    ///
    /// Example: (1-gram-counts, 2-grams-counts) or (2-grams-counts, 3-grams-counts)
    pub fn iter_tuple(&self) -> CountModelTupleIter {
        CountModelTupleIter {
            model: self,
            idx: 0,
        }
    }
}

/// Tuple iterator for successive count models
///
/// Example: (1-gram-counts, 2-grams-counts) or (2-grams-counts, 3-grams-counts)
///
/// # Fields
///
/// `model` - Count model holding various lenght ngram-counts
/// `idx` - Current iterator index
pub struct CountModelTupleIter<'a> {
    model: &'a CountModel,
    idx: usize,
}

impl<'a> Iterator for CountModelTupleIter<'a> {
    type Item = (&'a NGramModel, &'a NGramModel);

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        let prefix_model = match self.model.get_ngram_model(self.idx) {
            Some(model) => model,
            None => return None,
        };
        let ngram_model = match self.model.get_ngram_model(self.idx + 1) {
            Some(model) => model,
            None => return None,
        };
        Some((prefix_model, ngram_model))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use models::sigma::SigmaType;

    #[test]
    fn test_count_model1() {
        let sigma: Sigma = Sigma::new(None, SigmaType::Test);
        let ngram_length: usize = 2;
        let mut count_model = CountModel::from_sigma(&sigma, ngram_length).unwrap();
        let ngram_model: &mut NGramModel = count_model.get_mut_ngram_model(ngram_length).unwrap();
        ngram_model.add_ngram("aa").unwrap();
        assert_eq!(&1.0, ngram_model.get_ngram_count("aa").unwrap());
    }

    #[test]
    fn test_count_model2() {
        let sigma: Sigma = Sigma::new(None, SigmaType::Test);
        let ngram_length: usize = 2;
        let mut count_model = CountModel::from_sigma(&sigma, ngram_length).unwrap();
        let ngrams: Vec<String> = vec!["aa".to_string(), "aa".to_string(), "ab".to_string()];
        count_model.count_ngrams(&ngrams, ngram_length).unwrap();
        let ngram_model: &mut NGramModel = count_model.get_mut_ngram_model(ngram_length).unwrap();
        assert_eq!(&2.0, ngram_model.get_ngram_count("aa").unwrap());
        assert_eq!(&1.0, ngram_model.get_ngram_count("ab").unwrap());
    }
}
