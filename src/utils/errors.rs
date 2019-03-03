use std::error::Error;
use std::fmt;
use std::io::Error as IOError;

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
        let desc = format!("io::Error: {}", err.description());
        UtilError::new(&desc[..])
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for UtilError {
    fn from(err: std::boxed::Box<dyn std::error::Error>) -> Self {
        let desc = format!("{}", err.description());
        UtilError::new(&desc[..])
    }
}
