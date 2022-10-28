pub mod data;
pub mod environment;
pub mod error;
pub mod files;
pub mod numbers;
pub mod strings;
pub mod utils;
pub mod validate;

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

// Types ──────────────────────────────────────────────────────────────────── //

pub type ResultsInOk = Result<(), Box<dyn Error>>;
pub type ResultsIn<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct SimpleError {
    pub message: String,
}

impl SimpleError {
    pub fn new(message: &str) -> SimpleError {
        SimpleError {
            message: String::from(message),
        }
    }
    pub fn boxed(message: &str) -> Box<dyn Error> {
        Box::new(SimpleError::new(message))
    }
}

impl Error for SimpleError {}
impl Display for SimpleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}
