#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error: {description} - {source}")]
    Error {
        source: Box<dyn std::error::Error>,
        description: String,
    },

    #[error("Error: {description}")]
    GeneralError { description: String },

    #[error("Invalid Configuration: {description}")]
    InvalidConfiguration { description: String },

    #[error("Invalid Value: {description}")]
    InvalidValue { description: String },

    #[error("Not Found: '{name}', {description}")]
    NotFound { name: String, description: String },

    #[error("Timed Out: {description}")]
    TimedOut { description: String },

    #[error("User exit:")]
    UserExit,
}

impl Error {
    pub fn error(source: Box<dyn std::error::Error>, description: &str) -> Error {
        Error::Error {
            description: String::from(description),
            source,
        }
    }
    pub fn general(description: &str) -> Error {
        Error::GeneralError {
            description: String::from(description),
        }
    }
    pub fn invalid_configuration(description: &str) -> Error {
        Error::InvalidConfiguration {
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
}
