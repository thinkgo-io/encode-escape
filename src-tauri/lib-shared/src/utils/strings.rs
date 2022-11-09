use crate::prelude::*;

pub fn to_utf8(value: &[u8]) -> Result<String> {
    Ok(String::from_utf8(value.to_vec())?)
}

pub fn trim_not_empty(text: &str) -> Result<&str> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(Error::invalid_value("value is empty"));
    }
    Ok(trimmed)
}
