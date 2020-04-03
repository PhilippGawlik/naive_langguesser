use smoothing::errors::SmoothingError;
use std::error::Error;
use std::fmt;
use std::io::Error as IOError;
use utils::errors::UtilError;

#[derive(Debug)]
pub struct LanguageModelError {
    details: String,
}

impl LanguageModelError {
    pub fn new(msg: &str) -> LanguageModelError {
        LanguageModelError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for LanguageModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for LanguageModelError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<IOError> for LanguageModelError {
    fn from(err: IOError) -> Self {
        let desc = format!("io::Error: {}", err.to_string());
        LanguageModelError::new(&desc[..])
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for LanguageModelError {
    fn from(err: std::boxed::Box<dyn std::error::Error>) -> Self {
        let desc = format!("Error: {}", err.to_string());
        LanguageModelError::new(&desc[..])
    }
}

impl From<UtilError> for LanguageModelError {
    fn from(err: UtilError) -> Self {
        let desc = format!("UtilError: {}", err.to_string());
        LanguageModelError::new(&desc[..])
    }
}

impl From<SmoothingError> for LanguageModelError {
    fn from(err: SmoothingError) -> Self {
        let desc = format!("SmoothingError: {}", err.to_string());
        LanguageModelError::new(&desc[..])
    }
}

#[derive(Debug)]
pub struct InfererError {
    details: String,
}

impl InfererError {
    pub fn new(msg: &str) -> InfererError {
        InfererError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for InfererError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for InfererError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<IOError> for InfererError {
    fn from(err: IOError) -> Self {
        let desc = format!("io::Error: {}", err.to_string());
        InfererError::new(&desc[..])
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for InfererError {
    fn from(err: std::boxed::Box<dyn std::error::Error>) -> Self {
        let desc = format!("Error: {}", err.to_string());
        InfererError::new(&desc[..])
    }
}

impl From<UtilError> for InfererError {
    fn from(err: UtilError) -> Self {
        let desc = format!("UtilError: {}", err.to_string());
        InfererError::new(&desc[..])
    }
}
