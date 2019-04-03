pub enum Mode {
    Model,
    Guess,
}

pub enum Sigma {
    Test,
}

pub struct ModelConfig {
    pub filename: String,
    pub modelname: String,
    pub outpath: String,
    pub application_mode: Mode,
    pub sigma: Sigma,
    pub feature_length: usize,
}

pub struct GuessConfig {
    pub filename: String,
    pub application_mode: Mode,
}

impl ModelConfig {
    pub fn new(matches: &clap::ArgMatches) -> ModelConfig {
        // clap ensures existing value
        let filename = matches.value_of("path").unwrap().to_string();
        let modelname = matches.value_of("model-name").unwrap().to_string();
        let outpath = format!(
            "data/models/{}.model",
            matches.value_of("model-name").unwrap().to_string()
        );
        let application_mode = Mode::Model;
        let sigma: Sigma = match matches.value_of("alphabet").unwrap() {
            "test" => Sigma::Test,
            "ascii" => panic!("Alphabet ascii is not implemented"),
            "unicode" => panic!("Alphabet unicode is not implemented"),
            _ => panic!("Alphabet is not implemented"),
        };
        let feature_length = matches
            .value_of("n-gram-length")
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();
        return ModelConfig {
            filename,
            modelname,
            outpath,
            application_mode,
            sigma,
            feature_length,
        };
    }
}

impl GuessConfig {
    pub fn new(matches: &clap::ArgMatches) -> GuessConfig {
        // clap ensures existing value
        let filename = matches.value_of("path").unwrap().to_string();
        let application_mode = Mode::Guess;
        return GuessConfig {
            filename,
            application_mode,
        };
    }
}
