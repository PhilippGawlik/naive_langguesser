use inferer::errors::InfererError;
use models::probability_model::ProbabilityModel;
use models::text_model::TextModel;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use utils::{get_model_paths, sort_by_second_element};

pub mod errors;

/// Infer text language according to language models
/// Inferring is done based on text's ngrams of specified `ngram_length`.  Inferring will be done in parallel for the present `models` if `in_parallel` is true.
///
/// # Fields
///
/// * `models` - List of language models
/// * `ngram_length` - length of ngrams the inference is based on
/// * `in_parallel` - parallel processing flag
///
pub struct Inferer {
    models: Vec<ProbabilityModel>,
    ngram_length: usize,
    in_parallel: bool,
}

impl Inferer {
    /// Init from directory holding dumped probability models files
    pub fn from_models_dir(
        dir: &str,
        ngram_length: usize,
        in_parallel: bool,
    ) -> Result<Inferer, InfererError> {
        let model_paths = get_model_paths(dir)?;
        Inferer::from_model_files(model_paths, ngram_length, in_parallel)
    }

    /// Init from a list of paths
    ///
    /// Init by reading probability models from give paths
    pub fn from_model_files(
        model_paths: Vec<String>,
        ngram_length: usize,
        in_parallel: bool,
    ) -> Result<Inferer, InfererError> {
        let models = model_paths
            .into_iter()
            .map(|path| {
                let model = match ProbabilityModel::from_file(&path[..]) {
                    Ok(model) => model,
                    Err(err) => panic!("{} for file {}!", err, &path[..]),
                };
                model
            })
            .collect::<Vec<ProbabilityModel>>();
        Ok(Inferer {
            models,
            ngram_length,
            in_parallel,
        })
    }

    /// Infer most likely language for given text
    pub fn infer(self, unclassified: &TextModel) -> Result<Vec<(String, f64)>, InfererError> {
        let ngrams: Vec<String> = unclassified.get_ngrams(self.ngram_length);
        let mut prob_table: Vec<(String, f64)> = match self.in_parallel {
            true => self.parallel_infer(ngrams)?,
            false => self.successive_infer(&ngrams)?,
        };
        prob_table = sort_by_second_element(prob_table)?;
        Ok(prob_table)
    }

    /// Calculate likelihood of being of a specific language in parallel
    fn parallel_infer(self, ngrams: Vec<String>) -> Result<Vec<(String, f64)>, InfererError> {
        let shared_ngrams = Arc::new(ngrams);
        let (sender, receiver): (Sender<(String, f64)>, Receiver<(String, f64)>) = channel();
        for model in self.models {
            let sender_instance = sender.clone();
            let name: String = model.name.clone();
            let ngrams = Arc::clone(&shared_ngrams);
            thread::spawn(move || {
                let probability = match calculate_log_space_probability(&model, &ngrams) {
                    Ok(probability) => probability,
                    Err(err) => panic!("Thread can't calculate probability because of {}", err),
                };
                match sender_instance.send((name, probability)) {
                    Ok(_) => drop(sender_instance),
                    Err(err) => panic!("Thread couldn't send probability because of {}", err),
                };
            });
        }
        drop(sender);
        let mut prob_table: Vec<(String, f64)> = Vec::new();
        while let Ok(guess) = receiver.recv() {
            prob_table.push(guess);
        }
        Ok(prob_table)
    }

    /// Calculate likelihood of being of a specific language in successively
    fn successive_infer(&self, ngrams: &Vec<String>) -> Result<Vec<(String, f64)>, InfererError> {
        self.models
            .iter()
            .map(|model| {
                let name = model.name.clone();
                match calculate_log_space_probability(&model, &ngrams) {
                    Ok(probability) => Ok((name, probability)),
                    Err(err) => Err(err),
                }
            })
            .collect()
    }
}

/// Take product of ngrams occurence probabilities
///
/// Multiplication is done in logspace to avoid probabilities
/// that are too small for `f64` type.
/// Monotony of log operation ensures correct ranking of language
/// affiliation of the text.
pub fn calculate_log_space_probability(
    model: &ProbabilityModel,
    ngrams: &Vec<String>,
) -> Result<f64, InfererError> {
    let default: f64 = 1.0;
    let mut product = default.log2();
    for ngram in ngrams.iter() {
        let prob: f64 = match model.get(&ngram[..]) {
            Some(prob) => *prob,
            None => {
                // Todo possible backoff
                // Shouldn't occur cause ngram models are total
                return Err(InfererError::new(&format!("Unknown ngram: {}", ngram)[..]));
            }
        };
        product += prob.log2();
    }
    Ok(product)
}
