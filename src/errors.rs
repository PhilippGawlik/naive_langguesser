use models::errors::InfererError;
use models::errors::LanguageModelError;
use std::error::Error;
use std::fmt;
use std::io::Error as IOError;
use utils::errors::UtilError;

#[derive(Debug)]
pub struct ModellingError {
    details: String,
}

impl ModellingError {
    pub fn new(msg: &str) -> ModellingError {
        ModellingError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ModellingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ModellingError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<IOError> for ModellingError {
    fn from(err: IOError) -> Self {
        let desc = format!("io::Error: {}", err.to_string());
        ModellingError::new(&desc[..])
    }
}

impl From<LanguageModelError> for ModellingError {
    fn from(err: LanguageModelError) -> Self {
        let desc = format!("LanguageModelError: {}", err.description());
        ModellingError::new(&desc[..])
    }
}

#[derive(Debug)]
pub struct GuessingError {
    details: String,
}

impl GuessingError {
    pub fn new(msg: &str) -> GuessingError {
        GuessingError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for GuessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for GuessingError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<IOError> for GuessingError {
    fn from(err: IOError) -> Self {
        let desc = format!("io::Error: {}", err.to_string());
        GuessingError::new(&desc[..])
    }
}

impl From<LanguageModelError> for GuessingError {
    fn from(err: LanguageModelError) -> Self {
        let desc = format!("LanguageModelError: {}", err.to_string());
        GuessingError::new(&desc[..])
    }
}

impl From<InfererError> for GuessingError {
    fn from(err: InfererError) -> Self {
        let desc = format!("InfererError: {}", err.to_string());
        GuessingError::new(&desc[..])
    }
}

impl From<UtilError> for GuessingError {
    fn from(err: UtilError) -> Self {
        let desc = format!("UtilError: {}", err.to_string());
        GuessingError::new(&desc[..])
    }
}
