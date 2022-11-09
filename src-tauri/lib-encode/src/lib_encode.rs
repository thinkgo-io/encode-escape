pub mod base64;
pub mod html;
pub mod url;

use shared::prelude::*;

pub fn encode(encoding: &str, operation: &str, input: &str) -> Result<String> {
    match encoding {
        "base64" => encode_base_64(operation, input),
        "html" => encode_html(operation, input),
        "url" => encode_url(operation, input),
        "smart-url" => encode_url(operation, input),
        _ => to_invalid_value(&("[Uknown encoding '".to_string() + encoding + "']")),
    }
}

fn encode_base_64(operation: &str, input: &str) -> Result<String> {
    match operation {
        "encode" => Ok(base64::encode(input)),
        "decode" => base64::decode(input),
        _ => to_invalid_operation(operation),
    }
}

fn encode_html(operation: &str, input: &str) -> Result<String> {
    match operation {
        "escape" => Ok(html::encode(input)),
        "unescape" => html::decode(input),
        _ => to_invalid_operation(operation),
    }
}

fn encode_url(operation: &str, input: &str) -> Result<String> {
    match operation {
        "escape" => Ok(url::encode(input)),
        "unescape" => url::decode(input),
        _ => to_invalid_operation(operation),
    }
}

fn to_invalid_operation(operation: &str) -> Result<String> {
    Err(Error::invalid_value(&("[Uknown operation '".to_string() + operation + "']")))
}

fn to_invalid_value(message: &str) -> Result<String> {
    Err(Error::invalid_value(message))
}
