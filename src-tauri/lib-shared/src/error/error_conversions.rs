use crate::error::Error;

impl From<Box<dyn std::error::Error>> for Error {
    fn from(source: Box<dyn std::error::Error>) -> Self {
        Error::error("Error", source)
    }
}

impl From<std::env::VarError> for Error {
    fn from(source: std::env::VarError) -> Self {
        Error::error("Var Error", Box::new(source))
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error::error("IO Error", Box::new(source))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(source: std::num::ParseIntError) -> Self {
        Error::error("Parse Int Error", Box::new(source))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(source: std::string::FromUtf8Error) -> Self {
        Error::error("UTF8 Error", Box::new(source))
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Error::error("Serde JSON Error", Box::new(source))
    }
}

pub fn to_boxed_error(error: Error) -> Box<dyn std::error::Error> {
    Box::new(error)
}
