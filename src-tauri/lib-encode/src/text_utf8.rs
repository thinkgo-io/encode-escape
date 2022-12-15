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
    if contains_separators(&value) {
        hex_vec_to_string(split_on_separator(&value))
    } else {
        hex_vec_to_string(split_every(&value, 2))
    }
}

pub fn to_decimal(value: &str) -> String {
    let mut result = String::new();
    let mut first = First::new();
    for item in value.as_bytes() {
        if first.not_first() {
            result.push_str(" ");
        }
        result.push_str(&format!("{}", item));
    }
    result
}

pub fn to_hex(value: &str) -> String {
    let mut result = String::new();
    let mut first = First::new();
    for item in value.as_bytes() {
        if first.not_first() {
            result.push_str(" ");
        }
        result.push_str(&format!("{:02X}", item))
    }
    result
}

// Private ────────────────────────────────────────────────────────────────── //

fn bytes_to_string(bytes: Vec<u8>) -> Result<String> {
    String::from_utf8(bytes).map_err(|error| Error::InvalidValue {
        description: format!("Invalid UTF-8: {}", error),
    })
}

fn contains_separators(value: &str) -> bool {
    contains_one_of(value, " -_.")
}

fn decimal_to_byte(value: &str, position: isize) -> Result<u8> {
    radix_to_byte(value, position, 10, "Invalid decimal value")
}

fn decimal_vec_to_bytes(values: Vec<&str>) -> Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut counter = Counter::new();
    for value in values {
        if is_blank(value) {
            continue;
        }
        counter.increment();
        bytes.push(decimal_to_byte(value, counter.value)?);
    }
    Ok(bytes)
}

fn decimal_vec_to_string(values: Vec<&str>) -> Result<String> {
    let bytes = decimal_vec_to_bytes(values)?;
    bytes_to_string(bytes)
}

fn hex_to_byte(value: &str, position: isize) -> Result<u8> {
    radix_to_byte(value, position, 16, "Invalid hex value")
}

fn hex_vec_to_bytes(values: Vec<&str>) -> Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut counter = Counter::new();
    for value in values {
        if is_blank(value) {
            continue;
        }
        counter.increment();
        bytes.push(hex_to_byte(value, counter.value)?);
    }
    Ok(bytes)
}

fn hex_vec_to_string(values: Vec<&str>) -> Result<String> {
    let bytes = hex_vec_to_bytes(values)?;
    bytes_to_string(bytes)
}

fn invalid_value(value: &str, position: isize, message: &str) -> Error {
    Error::invalid_value(&format!(
        "{} '{}' at position {}.",
        message, value, position
    ))
}

fn radix_to_byte(value: &str, position: isize, radix: u32, invalid_message: &str) -> Result<u8> {
    u8::from_str_radix(value, radix).map_err(|_| invalid_value(&value, position, invalid_message))
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

    // From Decimal ───────────────────────────────────────── //

    #[test]
    fn test_from_decimal() {
        assert_eq!(from_decimal("84 101 115 116").unwrap(), "Test".to_string());
    }

    #[test]
    fn test_from_decimal_empty() {
        assert_eq!(from_decimal("").unwrap(), "".to_string());
    }

    //    #[test]
    //    fn test_from_hex_error() {
    //        assert_eq!(from_hex("z"), Ok("Test".to_string()));
    //    }

    #[test]
    fn test_from_decimal_with_unicode() {
        assert_eq!(
            from_decimal("42 32 240 159 153 130").unwrap(),
            "* 🙂".to_string()
        );
    }

    #[test]
    fn test_from_decimal_with_whitespace() {
        assert_eq!(
            from_decimal(" 84 101 115 116 ").unwrap(),
            "Test".to_string()
        );
    }

    // From Hex ───────────────────────────────────────────── //

    #[test]
    fn test_from_hex() {
        assert_eq!(from_hex("54 65 73 74").unwrap(), "Test".to_string());
    }

    #[test]
    fn test_from_hex_empty() {
        assert_eq!(from_hex("").unwrap(), "".to_string());
    }

    //    #[test]
    //    fn test_from_hex_error() {
    //        assert_eq!(from_decimal("z"), Ok("Test".to_string()));
    //    }

    #[test]
    fn test_from_hex_with_unicode() {
        assert_eq!(from_hex("2A 20 F0 9F 99 82").unwrap(), "* 🙂".to_string());
    }

    #[test]
    fn test_from_hex_with_whitespace() {
        assert_eq!(from_hex(" 54 65 73 74 ").unwrap(), "Test".to_string());
    }

    // To Decimal ─────────────────────────────────────────── //

    #[test]
    fn test_to_decimal() {
        assert_eq!(to_decimal("Test"), "84 101 115 116");
    }

    #[test]
    fn test_to_decimal_with_unicode() {
        assert_eq!(to_decimal("* 🙂"), "42 32 240 159 153 130");
    }

    #[test]
    fn test_to_decimal_empty() {
        assert_eq!(to_decimal(""), "");
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

    #[test]
    fn test_to_hex_unicode() {
        assert_eq!(to_hex("* 🙂"), "2A 20 F0 9F 99 82");
    }
}
