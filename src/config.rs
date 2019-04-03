pub enum Mode {
    Model,
    Guess,
}

pub enum Sigma {
    Test,
}

pub struct Config {
    pub filename: String,
    pub modelname: Option<String>,
    pub outpath: Option<String>,
    pub application_mode: Mode,
    pub sigma: Option<Sigma>,
    pub feature_length: Option<usize>,
}

impl Config {
    pub fn new(matches: clap::ArgMatches) -> Config {
        if let Some(model_matches) = matches.subcommand_matches("model") {
            let filename = model_matches
                .value_of("path")
                .unwrap() // clap ensures existing value
                .to_string();
            let modelname = Some(
                model_matches
                    .value_of("model-name")
                    .unwrap() // clap ensures existing value
                    .to_string(),
            );
            let outpath = Some(format!(
                "data/models/{}.model",
                model_matches
                    .value_of("model-name")
                    .unwrap() // clap ensures existing value
                    .to_string()
            ));
            let application_mode = Mode::Model;
            let sigma: Option<Sigma> = match model_matches.value_of("alphabet").unwrap() {
                "test" => Some(Sigma::Test),
                "ascii" => panic!("Alphabet ascii is not implemented"),
                "unicode" => panic!("Alphabet unicode is not implemented"),
                _ => panic!("Alphabet is not implemented"),
            };
            let feature_length = Some(
                model_matches
                    .value_of("n-gram-length")
                    .unwrap()
                    .to_string()
                    .parse::<usize>()
                    .unwrap(),
            );
            return Config {
                filename,
                modelname,
                outpath,
                application_mode,
                sigma,
                feature_length,
            };
        } else {
            let model_matches = matches.subcommand_matches("guess").unwrap();
            let filename = model_matches
                .value_of("path")
                .unwrap() // clap ensures existing value
                .to_string();
            let modelname = None;
            let outpath = None;
            let sigma = None;
            let feature_length = None;
            let application_mode = Mode::Guess;
            return Config {
                filename,
                modelname,
                outpath,
                application_mode,
                sigma,
                feature_length,
            };
        };
    }
}
