use shared::error::Error;
use shared::strings::to_utf8;
use shared::Result;
use base64::DecodeError;

pub fn decode(value: &str) -> Result<String> {
    let decoded = base64::decode(value).map_err(to_invalid_value)?;
    Ok(to_utf8(&decoded)?)
}

pub fn encode(value: &str) -> String {
    base64::encode(value)
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
