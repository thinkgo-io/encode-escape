use shellexpand::full;
use shellexpand::LookupError;
use std::env::var;
use std::env::VarError;

use crate::prelude::*;

pub fn environment(name: &str) -> Result<String> {
    Ok(var(name)?)
}

pub fn expand(value: &str) -> Result<String> {
    Ok(full(value)
        .map_err(|error| to_invalid_value(&error))?
        .into_owned())
}

fn to_invalid_value(error: &LookupError<VarError>) -> Error {
    Error::invalid_value(&format!("invalid value: {}", error))
}
