use shellexpand::full;
use shellexpand::LookupError;
use std::env::var;
use std::env::VarError;

use crate::prelude::*;

pub fn environment(name: &str) -> Result<String> {
    match var(name) {
        Ok(value) => Ok(value),
        Err(error) => Err(environment_invalid_value(&error, name)),
    }
}

pub fn expand(value: &str) -> Result<String> {
    match full(value) {
        Ok(expanded) => Ok(expanded.to_string()),
        Err(error) => Err(expand_invalid_value(&error)),
    }
}

fn invalid_value(operation: &str, value: &str, message: &str) -> Error {
    Error::invalid_value(&format!(
        "{} Error : '{}': '{:?}'",
        operation, value, message
    ))
}

fn environment_invalid_value(error: &VarError, value: &str) -> Error {
    invalid_value("Environment", value, &error.to_string())
}

fn expand_invalid_value<E: std::error::Error>(error: &LookupError<E>) -> Error {
    invalid_value("Expand", &error.var_name, &error.to_string())
}
