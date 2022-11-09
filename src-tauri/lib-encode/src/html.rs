use shared::prelude::*;

use html_escape::decode_html_entities;
use html_escape::encode_text;

pub fn decode(value: &str) -> Result<String> {
    Ok(decode_html_entities(value).into_owned().to_string())
}

pub fn encode(value: &str) -> String {
    encode_text(value).to_owned().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_encode() {
        let value = "Hello & Greetings <to the world>";
        let actual = encode(value);
        assert_eq!(actual, "Hello &amp; Greetings &lt;to the world&gt;");
    }

    #[test]
    fn test_decode() {
        let value = "Hello &amp; Greetings &lt;to the world&gt;";
        let actual = decode(value);
        assert_eq!(actual.unwrap(), "Hello & Greetings <to the world>");
    }

    #[test]
    fn test_decode_unescaped() {
        let value = "Hello & Greetings <to the world>";
        let actual = decode(value);
        assert_eq!(actual.unwrap(), "Hello & Greetings <to the world>");
    }
}
