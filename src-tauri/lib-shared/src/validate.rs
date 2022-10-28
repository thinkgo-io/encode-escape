use crate::data::result::TResult;
use crate::error::{error, invalid_value};
use std::error::Error;

pub fn check_no_error<T, E: Error>(result: &Result<T, E>) -> TResult<()> {
    match result {
        Ok(_) => Ok(()),
        Err(actual) => Err(error(actual.to_string())),
    }
}

pub fn check_not_empty(value: &str) -> TResult<()> {
    if value.is_empty() {
        return Err(invalid_value("value is empty".to_string()));
    }
    Ok(())
}

pub fn check_parsed_ok<T, E>(
    value: &str,
    result: &Result<T, E>,
    error_message: &str,
) -> TResult<()> {
    if result.is_err() {
        return Err(invalid_value(format!("'{}' {}.", value, error_message)));
    }
    Ok(())
}
