use crate::data::result::TResult;
use crate::error::invalid_value;
use crate::ResultsIn;

pub fn to_utf8(value: &[u8]) -> ResultsIn<String> {
    match String::from_utf8(value.to_vec()) {
        Ok(decoded) => Ok(decoded),
        Err(error) => Err(Box::new(error)),
    }
}

pub fn trim_not_empty(text: &str) -> TResult<&str> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(invalid_value("value is empty".to_string()));
    }
    Ok(trimmed)
}
