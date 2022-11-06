use std::fmt;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;
pub type ResultOk = std::result::Result<(), Error>;

#[derive(Debug)]
pub enum Error {
    Error {
        description: String,
        cause: Option<Box<dyn std::error::Error>>,
    },
    InvalidConfiguration {
        description: String,
    },
    InvalidData {
        description: String,
    },
    InvalidValue {
        description: String,
    },
    NotFound {
        description: String,
        name: String,
    },
    TimedOut {
        description: String,
    },
    UserExit,
}

impl Error {
    pub fn new(description: &str) -> Error {
        Error::Error {
            description: String::from(description),
            cause: None,
        }
    }
    pub fn with_cause(description: &str, cause: Box<dyn std::error::Error>) -> Error {
        Error::Error {
            description: String::from(description),
            cause: Some(cause),
        }
    }
    pub fn invalid_configuration(description: &str) -> Error {
        Error::InvalidConfiguration {
            description: String::from(description),
        }
    }
    pub fn invalid_data(description: &str) -> Error {
        Error::InvalidData {
            description: String::from(description),
        }
    }
    pub fn invalid_value(description: &str) -> Error {
        Error::InvalidValue {
            description: String::from(description),
        }
    }
    pub fn not_found(description: &str, name: &str) -> Error {
        Error::NotFound {
            description: String::from(description),
            name: String::from(name),
        }
    }
    pub fn timed_out(description: &str) -> Error {
        Error::TimedOut {
            description: String::from(description),
        }
    }
    pub fn user_exit() -> Error {
        Error::UserExit
    }

    pub fn description(&self) -> &str {
        match self {
            Error::Error { description, .. } => description,
            Error::InvalidConfiguration { description } => description,
            Error::InvalidData { description } => description,
            Error::InvalidValue { description } => description,
            Error::NotFound { description, .. } => description,
            Error::TimedOut { description } => description,
            Error::UserExit => "User exit",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Error::Error { description, cause } => {
                if let Some(cause) = cause {
                    format!("{}: {}", description, cause)
                } else {
                    description.clone()
                }
            }
            Error::InvalidConfiguration { description } => description.clone(),
            Error::InvalidData { description } => description.clone(),
            Error::InvalidValue { description } => description.clone(),
            Error::NotFound { description, name } => format!("{}: {}", description, name),
            Error::TimedOut { description } => description.clone(),
            Error::UserExit => String::from("User exit"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Error { description, cause } => {
                write!(f, "Error: {} - {:?}", description, cause)
            }
            Error::InvalidConfiguration { description } => {
                write!(f, "Invalid Configuration: {}", description)
            }
            Error::InvalidData { description } => write!(f, "Invalid Data: {}", description),
            Error::InvalidValue { description } => write!(f, "Invalid Value: {}", description),
            Error::NotFound { description, name } => {
                write!(f, "{}, Not Found: {}", name, description)
            }
            Error::TimedOut { description } => write!(f, "Timed Out: {}", description),
            Error::UserExit => write!(f, "User Exit"),
        }
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(cause: Box<dyn std::error::Error>) -> Self {
        Error::with_cause("Error", cause)
    }
}

impl From<std::env::VarError> for Error {
    fn from(cause: std::env::VarError) -> Self {
        Error::with_cause("Var Error", Box::new(cause))
    }
}

impl From<std::io::Error> for Error {
    fn from(cause: std::io::Error) -> Self {
        Error::with_cause("IO Error", Box::new(cause))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(cause: std::num::ParseIntError) -> Self {
        Error::with_cause("Parse Int Error", Box::new(cause))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(cause: std::string::FromUtf8Error) -> Self {
        Error::with_cause("From UTF8 Error", Box::new(cause))
    }
}
