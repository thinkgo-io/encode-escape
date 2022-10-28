use crate::error::error;
use crate::error::TError;
use std::error::Error;

pub type TResult<T> = Result<T, TError>;

// TODO: Check this is still releveant.
// TODO: Do better with the From trait?

pub fn to_tresult<T, E: Error>(result: Result<T, E>) -> TResult<T> {
    match result {
        Ok(value) => Ok(value),
        Err(actual) => Err(error(actual.to_string())),
    }
}
