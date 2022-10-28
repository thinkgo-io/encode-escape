pub mod base64;
pub mod html;
pub mod url;

use shared::ResultsIn;
use shared::SimpleError;

pub fn encode(encoding: &str, variant: &str, input: &str) -> ResultsIn<String> {
    match encoding {
        "base64" => encode_base_64(variant, input),
        "html" => encode_html(variant, input),
        "url" => encode_url(variant, input),
        "smart-url" => encode_url(variant, input),
        _ => to_error(&("[Uknown encoding '".to_string() + encoding + "']")),
    }
}

fn encode_base_64(variant: &str, input: &str) -> ResultsIn<String> {
    match variant {
        "encode" => Ok(base64::encode(input)),
        "decode" => base64::decode(input),
        _ => to_error(&("[Uknown variant '".to_string() + variant + "']")),
    }
}

fn encode_html(variant: &str, input: &str) -> ResultsIn<String> {
    match variant {
        "escape" => Ok(html::encode(input)),
        "unescape" => html::decode(input),
        _ => to_error(&("[Uknown variant '".to_string() + variant + "']")),
    }
}

fn encode_url(variant: &str, input: &str) -> ResultsIn<String> {
    match variant {
        "escape" => Ok(url::encode(input)),
        "unescape" => url::decode(input),
        _ => to_error(&("[Uknown variant '".to_string() + variant + "']")),
    }
}

fn to_error(message: &str) -> ResultsIn<String> {
    Err(SimpleError::boxed(message))
}
