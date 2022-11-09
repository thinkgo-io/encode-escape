use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;
pub type ResultOk = std::result::Result<(), Error>;
