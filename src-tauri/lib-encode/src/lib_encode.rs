pub mod base64;
pub mod constants;
pub mod html;
pub mod text;
pub mod text_character;
pub mod text_utf8;
pub mod types;
pub mod url;
pub mod url_address;

use shared::prelude::*;

use crate::constants::*;
use crate::types::EncodeOperation;

// Public ─────────────────────────────────────────────────────────────────── //

pub fn encode(encode_operation: &EncodeOperation, input: &str) -> Result<String> {
    match encode_operation.encoding.as_str() {
        BASE_64 => encode_base_64(&encode_operation.operation, input),
        HTML => encode_html(&encode_operation.operation, input),
        TEXT => encode_text(&encode_operation.operation, input),
        TEXT_CHARACTER => encode_text_character(&encode_operation.operation, input),
        TEXT_UTF8 => encode_text_utf8(&encode_operation.operation, input),
        URL => encode_url(&encode_operation.operation, input),
        URL_ADDRESS => encode_url_address(&encode_operation.operation, input),
        _ => to_invalid_value(
            &("[Uknown encoding '".to_string() + &encode_operation.encoding + "']"),
        ),
    }
}

pub fn reverse_encode(encode_operation: &EncodeOperation, input: &str) -> Result<String> {
    match encode_operation.encoding.as_str() {
        BASE_64 => reverse_encode_base_64(&encode_operation.operation, input),
        HTML => reverse_encode_html(&encode_operation.operation, input),
        TEXT => reverse_encode_text(&encode_operation.operation, input),
        TEXT_CHARACTER => reverse_encode_text_character(&encode_operation.operation, input),
        TEXT_UTF8 => reverse_encode_text_utf8(&encode_operation.operation, input),
        URL => reverse_encode_url(&encode_operation.operation, input),
        URL_ADDRESS => reverse_encode_url_address(&encode_operation.operation, input),
        _ => to_invalid_value(
            &("[Uknown encoding '".to_string() + &encode_operation.encoding + "']"),
        ),
    }
}

// Private ────────────────────────────────────────────────────────────────── //

fn encode_base_64(operation: &str, input: &str) -> Result<String> {
    match operation {
        ENCODE => Ok(base64::encode(input)),
        DECODE => base64::decode(input),
        _ => to_invalid_operation(operation),
    }
}

fn encode_html(operation: &str, input: &str) -> Result<String> {
    match operation {
        ESCAPE => Ok(html::encode(input)),
        UNESCAPE => html::decode(input),
        _ => to_invalid_operation(operation),
    }
}

fn encode_text(operation: &str, input: &str) -> Result<String> {
    match operation {
        ESCAPE => Ok(text::escape(input)),
        UNESCAPE => Ok(text::unescape(input)),
        _ => to_invalid_operation(operation),
    }
}

fn encode_text_character(operation: &str, input: &str) -> Result<String> {
    match operation {
        FROM_DECIMAL => text_character::from_decimal(input),
        FROM_HEX => text_character::from_hex(input),
        TO_DECIMAL => Ok(text_character::to_decimal(input)),
        TO_HEX => Ok(text_character::to_hex(input)),
        _ => to_invalid_operation(operation),
    }
}

fn encode_text_utf8(operation: &str, input: &str) -> Result<String> {
    match operation {
        FROM_DECIMAL => text_utf8::from_decimal(input),
        FROM_HEX => text_utf8::from_hex(input),
        TO_DECIMAL => Ok(text_utf8::to_decimal(input)),
        TO_HEX => Ok(text_utf8::to_hex(input)),
        _ => to_invalid_operation(operation),
    }
}

fn encode_url(operation: &str, input: &str) -> Result<String> {
    match operation {
        ESCAPE => Ok(url_address::encode(input)),
        UNESCAPE => url_address::decode(input),
        _ => to_invalid_operation(operation),
    }
}

fn encode_url_address(operation: &str, input: &str) -> Result<String> {
    match operation {
        ESCAPE => Ok(url_address::encode(input)),
        UNESCAPE => url_address::decode(input),
        _ => to_invalid_operation(operation),
    }
}

// Reverse ────────────────────────────────────────────────────────────────── //

fn reverse_encode_base_64(operation: &str, input: &str) -> Result<String> {
    match operation {
        ENCODE => base64::decode(input),
        DECODE => Ok(base64::encode(input)),
        _ => to_invalid_operation(operation),
    }
}

fn reverse_encode_html(operation: &str, input: &str) -> Result<String> {
    match operation {
        ESCAPE => html::decode(input),
        UNESCAPE => Ok(html::encode(input)),
        _ => to_invalid_operation(operation),
    }
}

fn reverse_encode_text(operation: &str, input: &str) -> Result<String> {
    match operation {
        ESCAPE => Ok(text::unescape(input)),
        UNESCAPE => Ok(text::escape(input)),
        _ => to_invalid_operation(operation),
    }
}

fn reverse_encode_text_character(operation: &str, input: &str) -> Result<String> {
    match operation {
        FROM_DECIMAL => Ok(text_character::to_decimal(input)),
        FROM_HEX => Ok(text_character::to_hex(input)),
        TO_DECIMAL => text_character::from_decimal(input),
        TO_HEX => text_character::from_hex(input),
        _ => to_invalid_operation(operation),
    }
}

fn reverse_encode_text_utf8(operation: &str, input: &str) -> Result<String> {
    match operation {
        FROM_DECIMAL => Ok(text_utf8::to_decimal(input)),
        FROM_HEX => Ok(text_utf8::to_hex(input)),
        TO_DECIMAL => text_utf8::from_decimal(input),
        TO_HEX => text_utf8::from_hex(input),
        _ => to_invalid_operation(operation),
    }
}

fn reverse_encode_url(operation: &str, input: &str) -> Result<String> {
    match operation {
        ESCAPE => url_address::decode(input),
        UNESCAPE => Ok(url_address::encode(input)),
        _ => to_invalid_operation(operation),
    }
}

fn reverse_encode_url_address(operation: &str, input: &str) -> Result<String> {
    match operation {
        ESCAPE => url_address::decode(input),
        UNESCAPE => Ok(url_address::encode(input)),
        _ => to_invalid_operation(operation),
    }
}

// Error Handling ─────────────────────────────────────────────────────────── //

fn to_invalid_operation(operation: &str) -> Result<String> {
    Err(Error::invalid_value(
        &("[Uknown operation '".to_string() + operation + "']"),
    ))
}

fn to_invalid_value(message: &str) -> Result<String> {
    Err(Error::invalid_value(message))
}
