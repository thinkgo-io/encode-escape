use base64::DecodeError;
use shared::prelude::*;
use shared::utils::strings::remove_newlines;
use shared::utils::strings::to_utf8;
use shared::utils::First;

pub fn decode(value: &str) -> Result<String> {
    let decoded = base64::decode(remove_newlines(value)).map_err(to_invalid_value)?;
    let utf8 = to_utf8(&decoded);
    if utf8.is_ok() {
        utf8
    } else {
        Ok(to_hex(&decoded))
    }
}

pub fn encode(value: &str) -> String {
    base64::encode(value)
}

pub fn to_hex(values: &Vec<u8>) -> String {
    let mut result = String::new();
    let mut first = First::new();
    for item in values {
        if first.not_first() {
            result.push_str(" ");
        }
        result.push_str(&format!("{:02X}", item))
    }
    result
}

fn to_invalid_value(error: DecodeError) -> Error {
    Error::invalid_value(&("Decode Error: ".to_string() + error.to_string().as_str()))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_encode() {
        let value = "Hello World";
        let actual = encode(value);
        assert_eq!(actual, "SGVsbG8gV29ybGQ=");
    }

    #[test]
    fn test_decode() {
        let value = "SGVsbG8gV29ybGQ=";
        let actual = decode(value);
        assert_eq!(actual.unwrap(), "Hello World");
    }
}
