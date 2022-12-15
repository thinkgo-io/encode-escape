use regex::Regex;

use shared::prelude::*;
use shared::utils::strings::*;
use shared::utils::Counter;
use shared::utils::First;
use shared::validate::check_contains_only;

use crate::constants::*;

// Publing ────────────────────────────────────────────────────────────────── //

pub fn from_decimal(value: &str) -> Result<String> {
    if is_blank(value) {
        return Ok(String::new());
    }
    validate_decimal_values(value)?;
    decimal_vec_to_string(split_on_separator(&value))
}

pub fn from_hex(value: &str) -> Result<String> {
    if is_blank(value) {
        return Ok(String::new());
    }
    validate_hex_values(value)?;
    if contains_separators(&value) || value.chars().count() <= 6 {
        hex_vec_to_string(split_on_separator(&value))
    } else {
        hex_vec_to_string(split_every(&value, 2))
    }
}

pub fn to_decimal(value: &str) -> String {
    let mut result = String::new();
    let mut first = First::new();
    for item in value.chars() {
        if first.not_first() {
            result.push_str(" ");
        }
        result.push_str(&format!("{}", item as u32));
    }
    result
}

pub fn to_hex(value: &str) -> String {
    let mut result = String::new();
    let mut first = First::new();
    for item in value.chars() {
        if first.not_first() {
            result.push_str(" ");
        }
        let item = item as u32;
        match item {
            0..=255 => result.push_str(&format!("{:02X}", item)),
            256..=65535 => result.push_str(&format!("{:04X}", item)),
            _ => result.push_str(&format!("{:X}", item)),
        }
    }
    result
}

// Private ────────────────────────────────────────────────────────────────── //

fn contains_separators(value: &str) -> bool {
    contains_one_of(value, " -_.")
}

fn decimal_vec_to_string(values: Vec<&str>) -> Result<String> {
    let mut result = String::new();
    let mut counter = Counter::new();
    for value in values {
        if is_blank(value) {
            continue;
        }
        counter.increment();
        result.push(decimal_to_char(value, counter.value)?);
    }
    Ok(result)
}

fn decimal_to_char(value: &str, position: isize) -> Result<char> {
    radix_to_char(
        value,
        position,
        10,
        "Invalid decimal value",
        "Invalid decimal value - no character found",
    )
}

fn hex_vec_to_string(values: Vec<&str>) -> Result<String> {
    let mut result = String::new();
    let mut counter = Counter::new();
    for value in values {
        if is_blank(value) {
            continue;
        }
        counter.increment();
        result.push(hex_to_char(value, counter.value)?);
    }
    Ok(result)
}

fn hex_to_char(value: &str, position: isize) -> Result<char> {
    radix_to_char(
        value,
        position,
        16,
        "Invalid hex value",
        "Invalid hex value - no character found",
    )
}

fn invalid_value(value: &str, position: isize, message: &str) -> Error {
    Error::invalid_value(&format!(
        "{} '{}' at position {}.",
        message, value, position
    ))
}

fn not_printable(value: u32) -> bool {
    value <= 6 || (value >= 14 && value <= 31)
}

fn radix_to_char(
    value: &str,
    position: isize,
    radix: u32,
    invalid_message: &str,
    none_message: &str,
) -> Result<char> {
    let integer = u32::from_str_radix(value, radix)
        .map_err(|_| invalid_value(&value, position, invalid_message))?;

    if not_printable(integer) {
        return Ok('?');
    }
    match char::from_u32(integer) {
        Some(character) => Ok(character),
        None => Err(invalid_value(&value, position, none_message)),
    }
}

fn split_on_separator(value: &str) -> Vec<&str> {
    let regex = Regex::new(r"[\s\-\._]+").unwrap();
    regex.split(value).collect()
}

fn validate_decimal_values(value: &str) -> Result<()> {
    validate_values(value, VALID_DECIMAL_VALUES, "Invalid decimal values.")
}

fn validate_hex_values(value: &str) -> Result<()> {
    validate_values(value, VALID_HEX_VALUES, "Invalid hex values.")
}

fn validate_values(value: &str, allowed: &str, invalid_message: &str) -> Result<()> {
    match check_contains_only(value, allowed) {
        Some(invalid) => Err(Error::invalid_value(&format!(
            "{} {}",
            invalid_message, invalid.details
        ))),
        None => Ok(()),
    }
}

// Tests ──────────────────────────────────────────────────────────────────── //

#[cfg(test)]
mod test {
    use super::*;

    // From Hex ───────────────────────────────────────────── //

    #[test]
    fn test_from_hex() {
        assert_eq!(from_hex("54 65 73 74").unwrap(), "Test".to_string());
    }

    //    #[test]
    //    fn test_from_hex_error() {
    //        assert_eq!(from_hex("z"), Err(Error::invalid_value("Invalid hex values. z")));
    //    }

    #[test]
    fn test_from_hex_with_unicode() {
        assert_eq!(from_hex("2A 20 1F642").unwrap(), "* 🙂".to_string());
    }

    #[test]
    fn test_from_hex_with_whitespace() {
        assert_eq!(from_hex(" 54 65 73 74 ").unwrap(), "Test".to_string());
    }

    // To Hex ─────────────────────────────────────────────── //

    #[test]
    fn test_to_hex() {
        assert_eq!(to_hex("Test"), "54 65 73 74");
    }

    #[test]
    fn test_to_hex_empty() {
        assert_eq!(to_hex(""), "");
    }

    // To Decimal ─────────────────────────────────────────── //

    #[test]
    fn test_to_decimal() {
        assert_eq!(to_decimal("Test"), "84 101 115 116");
    }

    #[test]
    fn test_to_decimal_empty() {
        assert_eq!(to_decimal(""), "");
    }
}
