use shared::strings::to_utf8;
use shared::ResultsIn;

pub fn decode(value: &str) -> ResultsIn<String> {
    match base64::decode(value) {
        Ok(decoded) => to_utf8(&decoded),
        Err(error) => Err(Box::new(error)),
    }
}

pub fn encode(value: &str) -> String {
    base64::encode(value)
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
