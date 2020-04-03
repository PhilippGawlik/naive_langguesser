use std::error::Error;
use std::fmt;
use std::io::Error as IOError;
use utils::errors::UtilError;


#[derive(Debug)]
pub struct TextError {
    details: String,
}

impl TextError {
    pub fn new(msg: &str) -> TextError {
        TextError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for TextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for TextError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<IOError> for TextError {
    fn from(err: IOError) -> Self {
        let desc = format!("io::Error: {}", err.to_string());
        TextError::new(&desc[..])
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for TextError {
    fn from(err: std::boxed::Box<dyn std::error::Error>) -> Self {
        let desc = format!("Error: {}", err.to_string());
        TextError::new(&desc[..])
    }
}

impl From<UtilError> for TextError {
    fn from(err: UtilError) -> Self {
        let desc = format!("UtilError: {}", err.to_string());
        TextError::new(&desc[..])
    }
}
