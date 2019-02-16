pub enum Mode {
    Model,
    Guess
}

pub struct Config {
    pub filename: String,
    pub modelname: Option<String>,
    pub outpath: Option<String>,
    pub application_mode: Mode
}

impl Config {
    pub fn new(matches: &clap::ArgMatches) -> Result<Config, &'static str> {
        if let Some(model_matches) = matches.subcommand_matches("model") {
            let filename = model_matches
                .value_of("path")
                .unwrap()  // clap ensures existing value
                .to_string();
            let modelname = Some(model_matches
                .value_of("model-name")
                .unwrap()  // clap ensures existing value
                .to_string());
            let outpath = Some(format!( "data/models/{}.model", model_matches
                .value_of("model-name")
                .unwrap()  // clap ensures existing value
                .to_string()));
            let application_mode = Mode::Model;
            return Ok(Config{filename, modelname, outpath, application_mode})
        } else {
            let model_matches = matches.subcommand_matches("guess").unwrap();
            let filename = model_matches
                .value_of("path")
                .unwrap()  // clap ensures existing value
                .to_string();
            let modelname = None;
            let outpath = None;
            let application_mode = Mode::Guess;
            return Ok(Config{filename, modelname, outpath, application_mode})
        };
    }
}
