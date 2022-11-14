use crate::error::Error;

impl From<Box<dyn std::error::Error>> for Error {
    fn from(source: Box<dyn std::error::Error>) -> Self {
        Error::error(source, "Error")
    }
}

impl From<std::env::VarError> for Error {
    fn from(source: std::env::VarError) -> Self {
        Error::error(Box::new(source), "Var Error")
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error::error(Box::new(source), "IO Error")
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(source: std::num::ParseIntError) -> Self {
        Error::error(Box::new(source), "Parse Int Error")
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(source: std::string::FromUtf8Error) -> Self {
        Error::error(Box::new(source), "UTF8 Error")
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Error::error(Box::new(source), "Serde JSON Error")
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(source: serde_yaml::Error) -> Self {
        Error::error(Box::new(source), "Serde YAML Error")
    }
}

pub fn to_boxed_error(error: Error) -> Box<dyn std::error::Error> {
    Box::new(error)
}
