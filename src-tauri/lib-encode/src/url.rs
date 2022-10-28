use shared::ResultsIn;

pub fn decode(value: &str) -> ResultsIn<String> {
    match urlencoding::decode(value) {
        Ok(decoded) => Ok(decoded.into_owned()),
        Err(error) => Err(Box::new(error)),
    }
}

pub fn encode(value: &str) -> String {
    urlencoding::encode(value).to_owned().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_encode() {
        let value = "Hello World";
        let actual = encode(value);
        assert_eq!(actual, "Hello%20World");
    }

    #[test]
    fn test_decode() {
        let value = "Hello%20World";
        let actual = decode(value);
        assert_eq!(actual.unwrap(), "Hello World");
    }
}