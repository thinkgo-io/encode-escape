use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;
pub type ResultOk = std::result::Result<(), Error>;
pub type StandardBoxedResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type StandardBoxedResultOk = std::result::Result<(), Box<dyn std::error::Error>>;
