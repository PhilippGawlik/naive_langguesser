use models::errors::CountModelError;
use models::errors::{ProbabilityModelError, TextError};
use std::error::Error;
use std::fmt;
use std::io::Error as IOError;

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
        let desc = format!("File reading error: {}", err.to_string());
        ModellingError::new(&desc[..])
    }
}

impl From<TextError> for ModellingError {
    fn from(err: TextError) -> Self {
        let desc = format!("Text processing error: {}", err.to_string());
        ModellingError::new(&desc[..])
    }
}

impl From<CountModelError> for ModellingError {
    fn from(err: CountModelError) -> Self {
        let desc = format!("Counting error: {}", err.to_string());
        ModellingError::new(&desc[..])
    }
}

impl From<ProbabilityModelError> for ModellingError {
    fn from(err: ProbabilityModelError) -> Self {
        let desc = format!("Error during probability calculation: {}", err.to_string());
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

impl From<InfererError> for GuessingError {
    fn from(err: InfererError) -> Self {
        let desc = format!("InfererError: {}", err.to_string());
        GuessingError::new(&desc[..])
    }
}

impl From<TextError> for GuessingError {
    fn from(err: TextError) -> Self {
        let desc = format!("Text processing error: {}", err.to_string());
        GuessingError::new(&desc[..])
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

impl From<UtilError> for InfererError {
    fn from(err: UtilError) -> Self {
        let desc = format!("UtilError: {}", err.to_string());
        InfererError::new(&desc[..])
    }
}

impl From<ProbabilityModelError> for InfererError {
    fn from(err: ProbabilityModelError) -> Self {
        let desc = format!("ProbabilityModelError: {}", err.to_string());
        InfererError::new(&desc[..])
    }
}


#[derive(Debug)]
pub struct UtilError {
    details: String
}

impl UtilError {
    pub fn new(msg: &str) -> UtilError {
        UtilError{details: msg.to_string()}
    }
}

impl fmt::Display for UtilError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for UtilError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<IOError> for UtilError {
    fn from(err: IOError) -> Self {
        let desc = format!("io::Error: {}", err.to_string());
        UtilError::new(&desc[..])
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for UtilError {
    fn from(err: std::boxed::Box<dyn std::error::Error>) -> Self {
        let desc = format!("{}", err.to_string());
        UtilError::new(&desc[..])
    }
}


#[derive(Debug)]
pub struct SmoothingError {
    details: String,
}

impl SmoothingError {
    pub fn new(msg: &str) -> SmoothingError {
        SmoothingError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for SmoothingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for SmoothingError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<IOError> for SmoothingError {
    fn from(err: IOError) -> Self {
        let desc = format!("io::Error: {}", err.to_string());
        SmoothingError::new(&desc[..])
    }
}

impl From<UtilError> for SmoothingError {
    fn from(err: UtilError) -> Self {
        let desc = format!("UtilError: {}", err.to_string());
        SmoothingError::new(&desc[..])
    }
}
