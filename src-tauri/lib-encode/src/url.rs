use shared::prelude::*;

pub fn decode(value: &str) -> Result<String> {
    let decoded = urlencoding::decode(value)?;
    Ok(decoded.into_owned())
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