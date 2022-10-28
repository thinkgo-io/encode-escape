use std::fmt;
use std::fmt::{Display, Formatter};

pub enum TError {
    Error { description: String },
    InvalidConfiguration { description: String },
    InvalidData { description: String },
    InvalidValue { description: String },
    NotFound { name: String, description: String },
    UserExit,
}

impl Display for TError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TError::Error { description } => write!(f, "Error: {}", description),
            TError::InvalidConfiguration { description } => {
                write!(f, "Invalid Configuration: {}", description)
            }
            TError::InvalidData { description } => write!(f, "Invalid Data: {}", description),
            TError::InvalidValue { description } => write!(f, "Invalid Value: {}", description),
            TError::NotFound { name, description } => {
                write!(f, "{}, Not Found: {}", name, description)
            }
            TError::UserExit => write!(f, "User Exit"),
        }
    }
}

pub fn error(description: String) -> TError {
    TError::Error { description }
}

pub fn invalid_configuration(description: String) -> TError {
    TError::InvalidData { description }
}

pub fn invalid_data(description: String) -> TError {
    TError::InvalidData { description }
}

pub fn invalid_value(description: String) -> TError {
    TError::InvalidValue { description }
}

pub fn not_found(name: String, description: String) -> TError {
    TError::NotFound { name, description }
}

pub fn user_exit() -> TError {
    TError::UserExit
}
