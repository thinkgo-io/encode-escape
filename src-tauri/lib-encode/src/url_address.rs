use shared::prelude::*;

use url::Url;

const EMPTY_STRING: &str = "";
const ID_SEPARATOR: &str = "@";
const QUERY_SEPARATOR: &str = "?";
const FRAGMENT_SEPARATOR: &str = "#";

pub fn decode(value: &str) -> Result<String> {
    let decoded = urlencoding::decode(value)?;
    Ok(decoded.into_owned())
}

pub fn encode(value: &str) -> String {
    match Url::parse(value) {
        Ok(url) => encode_url(&url),
        Err(_) => into_encoded(value),
    }
}

fn encode_url(url: &Url) -> String {
    [
        url.scheme(),
        "://",
        url.username(),
        &into_password(url.password()),
        &id_separator(url),
        &into_string(url.host_str()),
        &into_port(url.port()),
        url.path(),
        query_separator(url),
        &into_string(url.query()),
        fragment_separator(url),
        &into_string(url.fragment()),
    ]
    .concat()
}

fn fragment_separator(url: &Url) -> &str {
    if url.fragment().is_none() {
        return EMPTY_STRING;
    }
    FRAGMENT_SEPARATOR
}

fn id_separator(url: &Url) -> &str {
    if url.username().is_empty() && url.password().is_none() {
        return EMPTY_STRING;
    }
    ID_SEPARATOR
}

fn query_separator(url: &Url) -> &str {
    if url.query().is_none() {
        return EMPTY_STRING;
    }
    QUERY_SEPARATOR
}

fn into_encoded(value: &str) -> String {
    urlencoding::encode(value).to_owned().to_string()
}

fn into_password(value: Option<&str>) -> String {
    match value {
        Some(password) => format!(":{}", password),
        None => EMPTY_STRING.to_string(),
    }
}

fn into_port(port: Option<u16>) -> String {
    match port {
        Some(port) => format!(":{}", port),
        None => EMPTY_STRING.to_string(),
    }
}

fn into_string(value: Option<&str>) -> String {
    value.unwrap_or("").to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_encode_raw() {
        let value = "Hello World";
        let actual = encode(value);
        assert_eq!(actual, "Hello%20World");
    }

    #[test]
    fn test_encode_with_all() {
        let value = "https://username:password@host.com:8080/dir/page?param=value#section";
        let actual = encode(value);
        assert_eq!(
            actual,
            "https://username:password@host.com:8080/dir/page?param=value#section"
        );
    }

    #[test]
    fn test_encode_with_all_with_escapes() {
        let value =
            "https://username <:password <@host.com:8080/dir </page <?param=value <#section <";
        let actual = encode(value);
        assert_eq!(
            actual,
            "https://username%20%3C:password%20%3C@host.com:8080/dir%20%3C/page%20%3C?param=value%20%3C#section%20%3C"
        );
    }

    //    #[test]
    //    fn test_encode_url_with_host_only() {
    //        let value = "http://speedsheet.io";
    //        let actual = encode(value);
    //        assert_eq!(actual, "http://speedsheet.io");
    //    }

    #[test]
    fn test_encode_url_with_path() {
        let value = "https://speedsheet.io/s/rust";
        let actual = encode(value);
        assert_eq!(actual, "https://speedsheet.io/s/rust");
    }

    #[test]
    fn test_encode_url_with_query_and_fragment() {
        let value = "https://speedsheet.io/s/rust?q=bool#boolean";
        let actual = encode(value);
        assert_eq!(actual, "https://speedsheet.io/s/rust?q=bool#boolean");
    }

    #[test]
    fn test_decode() {
        let value = "Hello%20World";
        let actual = decode(value);
        assert_eq!(actual.unwrap(), "Hello World");
    }
}
