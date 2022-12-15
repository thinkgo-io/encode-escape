use crate::prelude::*;

pub fn contains_one_of(value: &str, chars: &str) -> bool {
    for c in chars.chars() {
        if value.contains(c) {
            return true;
        }
    }
    false
}

pub fn is_blank(value: &str) -> bool {
    value.trim().is_empty()
}

pub fn split_every(value: &str, length: usize) -> Vec<&str> {
    let mut result = Vec::new();
    let mut index = 0;
    while index < value.len() {
        let end = index + length;
        if end > value.len() {
            result.push(&value[index..]);
            break;
        }
        result.push(&value[index..end]);
        index = end;
    }
    result
}

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
