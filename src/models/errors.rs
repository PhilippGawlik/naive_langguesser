use smoothing::errors::SmoothingError;
use std::error::Error;
use std::fmt;
use std::io::Error as IOError;
use utils::errors::UtilError;
use text_processing::errors::TextError;

#[derive(Debug)]
pub struct NGramModelError {
    details: String,
}

impl NGramModelError {
    pub fn new(msg: &str) -> NGramModelError {
        NGramModelError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for NGramModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for NGramModelError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<IOError> for NGramModelError {
    fn from(err: IOError) -> Self {
        let desc = format!("io::Error: {}", err.to_string());
        NGramModelError::new(&desc[..])
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for NGramModelError {
    fn from(err: std::boxed::Box<dyn std::error::Error>) -> Self {
        let desc = format!("Error: {}", err.to_string());
        NGramModelError::new(&desc[..])
    }
}

impl From<UtilError> for NGramModelError {
    fn from(err: UtilError) -> Self {
        let desc = format!("UtilError: {}", err.to_string());
        NGramModelError::new(&desc[..])
    }
}

impl From<TextError> for NGramModelError {
    fn from(err: TextError) -> Self {
        let desc = format!("TextError: {}", err.to_string());
        NGramModelError::new(&desc[..])
    }
}

impl From<SmoothingError> for NGramModelError {
    fn from(err: SmoothingError) -> Self {
        let desc = format!("SmoothingError: {}", err.to_string());
        NGramModelError::new(&desc[..])
    }
}

#[derive(Debug)]
pub struct ProbabilityModelError {
    details: String,
}

impl ProbabilityModelError {
    pub fn new(msg: &str) -> ProbabilityModelError {
        ProbabilityModelError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ProbabilityModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ProbabilityModelError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<IOError> for ProbabilityModelError {
    fn from(err: IOError) -> Self {
        let desc = format!("io::Error: {}", err.to_string());
        ProbabilityModelError::new(&desc[..])
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for ProbabilityModelError {
    fn from(err: std::boxed::Box<dyn std::error::Error>) -> Self {
        let desc = format!("Error: {}", err.to_string());
        ProbabilityModelError::new(&desc[..])
    }
}

impl From<UtilError> for ProbabilityModelError {
    fn from(err: UtilError) -> Self {
        let desc = format!("UtilError: {}", err.to_string());
        ProbabilityModelError::new(&desc[..])
    }
}

impl From<CountModelError> for ProbabilityModelError {
    fn from(err: CountModelError) -> Self {
        let desc = format!("CountModelError: {}", err.to_string());
        ProbabilityModelError::new(&desc[..])
    }
}

impl From<TextError> for ProbabilityModelError {
    fn from(err: TextError) -> Self {
        let desc = format!("TextError: {}", err.to_string());
        ProbabilityModelError::new(&desc[..])
    }
}
impl From<SmoothingError> for ProbabilityModelError {
    fn from(err: SmoothingError) -> Self {
        let desc = format!("SmoothingError: {}", err.to_string());
        ProbabilityModelError::new(&desc[..])
    }
}

#[derive(Debug)]
pub struct CountModelError {
    details: String,
}

impl CountModelError {
    pub fn new(msg: &str) -> CountModelError {
        CountModelError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for CountModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for CountModelError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<TextError> for CountModelError {
    fn from(err: TextError) -> Self {
        let desc = format!("TextError: {}", err.to_string());
        CountModelError::new(&desc[..])
    }
}

impl From<IOError> for CountModelError {
    fn from(err: IOError) -> Self {
        let desc = format!("io::Error: {}", err.to_string());
        CountModelError::new(&desc[..])
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for CountModelError {
    fn from(err: std::boxed::Box<dyn std::error::Error>) -> Self {
        let desc = format!("Error: {}", err.to_string());
        CountModelError::new(&desc[..])
    }
}

impl From<UtilError> for CountModelError {
    fn from(err: UtilError) -> Self {
        let desc = format!("UtilError: {}", err.to_string());
        CountModelError::new(&desc[..])
    }
}

impl From<SmoothingError> for CountModelError {
    fn from(err: SmoothingError) -> Self {
        let desc = format!("SmoothingError: {}", err.to_string());
        CountModelError::new(&desc[..])
    }
}

impl From<NGramModelError> for CountModelError {
    fn from(err: NGramModelError) -> Self {
        let desc = format!("NGramModelError: {}", err.to_string());
        CountModelError::new(&desc[..])
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
