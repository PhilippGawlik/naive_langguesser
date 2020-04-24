use smoothing::SmoothingType;
use text_processing::SigmaID;
use Mode;

/// Hold configuration for `Model` mode
///
/// # Fields
///
/// * `filename` - path to the the file holding a text example of the language
/// * `modelname` - name of the language model (relevant for name of language file representation)
/// * `outpath` - folder path to write the language model to
/// * `application_mode` - `Model` mode setting
/// * `sigma_id` - specification for preprocessing the text example file
/// * `ngram_length` - max ngram length for calculating the language model
/// * `set_marker` - if set, add text begin/end marker (flag and marker symbol in one field)
/// * `smoothing_type` - set type of smoothing for ngram counts
pub struct ModelConfig {
    pub filename: String,
    pub modelname: String,
    pub outpath: String,
    pub application_mode: Mode,
    pub sigma_id: SigmaID,
    pub ngram_length: usize,
    pub set_marker: Option<&'static str>,
    pub smoothing_type: SmoothingType,
}

impl ModelConfig {
    /// Collect and parse cli arguments of `Model` mode
    ///
    /// # Arguments
    ///
    /// * `matches` - `Clap` references holding cli arguments
    pub fn new(matches: &clap::ArgMatches) -> ModelConfig {
        let filename = matches.value_of("path").unwrap().to_string();
        let modelname = matches.value_of("model-name").unwrap().to_string();
        let outpath = format!(
            "data/models/{}.model",
            matches.value_of("model-name").unwrap().to_string()
        );
        let application_mode = Mode::Model;
        let sigma_id: SigmaID = match matches.value_of("alphabet").unwrap() {
            "test" => SigmaID::Test,
            "ascii" => SigmaID::Ascii,
            "unicode" => SigmaID::Unicode,
            _ => panic!("Alphabet is not implemented"),
        };
        let ngram_length = matches
            .value_of("n-gram-length")
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();
        let marker_symbol = "#";
        let set_marker: Option<&'static str> = match matches.is_present("set-marker") {
            true => Some(marker_symbol),
            false => None,
        };
        let smoothing_type: SmoothingType = match matches.value_of("smoothing-type").unwrap() {
            "no" => SmoothingType::NoSmoothing,
            "add_one" => SmoothingType::AddOneSmoothing,
            "witten_bell" => SmoothingType::WittenBellSmoothing,
            _ => panic!("Smoothing type is unknown"),
        };
        return ModelConfig {
            filename,
            modelname,
            outpath,
            application_mode,
            sigma_id,
            ngram_length,
            set_marker,
            smoothing_type,
        };
    }
}

/// Hold configuration for `Guess` mode
///
/// # Fields
///
/// * `filename` - path to the the file holding a text for language classification
/// * `model_dir` - directory holding present language models
/// * `application_mode` - `Guess` mode setting
/// * `sigma_id` - specification for preprocessing the text file
/// * `ngram_length` - max ngram length for text language classification
/// * `set_marker` - if set, add text begin/end marker (flag and marker symbol in one field)
/// * `in_parallel` - if set, causes parallel language model evaluation of text
pub struct GuessConfig {
    pub filename: String,
    pub model_dir: &'static str,
    pub application_mode: Mode,
    pub sigma_id: SigmaID,
    pub ngram_length: usize,
    pub set_marker: Option<&'static str>,
    pub in_parallel: bool,
}

impl GuessConfig {
    /// Collect and parse cli arguments of `Model` mode
    ///
    /// # Arguments
    ///
    /// * `matches` - `Clap` references holding cli arguments
    pub fn new(matches: &clap::ArgMatches) -> GuessConfig {
        let filename = matches.value_of("path").unwrap().to_string();
        let model_dir = "./data/models/";
        let application_mode = Mode::Guess;
        let sigma_id: SigmaID = match matches.value_of("alphabet").unwrap() {
            "test" => SigmaID::Test,
            "ascii" => SigmaID::Ascii,
            "unicode" => SigmaID::Unicode,
            _ => panic!("Alphabet is not implemented"),
        };
        let ngram_length = matches
            .value_of("n-gram-length")
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();
        let marker_symbol = "#";
        let set_marker: Option<&'static str> = match matches.is_present("set-marker") {
            true => Some(marker_symbol),
            false => None,
        };
        let in_parallel: bool = matches.is_present("in-parallel");
        return GuessConfig {
            filename,
            model_dir,
            application_mode,
            sigma_id,
            ngram_length,
            set_marker,
            in_parallel,
        };
    }
}
