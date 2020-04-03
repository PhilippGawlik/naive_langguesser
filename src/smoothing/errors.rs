use std::error::Error;
use std::fmt;
use std::io::Error as IOError;
use utils::errors::UtilError;

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
