use std::fmt;
use std::error::Error;
use utils::errors::UtilError;
use models::errors::ProbabilityModelError;


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
