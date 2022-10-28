use tauri::command;

use encode::encode;

use crate::constants;
use crate::log::*;
use crate::types::Encoding;

#[command]
pub fn get_encodings() -> Vec<Encoding> {
    log("Get Encodings");

    constants::get_encodings()
}

#[command]
pub fn on_encode(encoding: &str, variant: &str, input: &str) -> String {
    log_lines(vec![
        "On Encoding:",
        &format!("  Encoding: {}", encoding),
        &format!("  Variant:  {}", variant),
        &format!("  Input:    {}", input),
    ]);

    match encode(&encoding.to_lowercase(), &variant.to_lowercase(), input) {
        Ok(output) => output,
        Err(error) => error.to_string(),
    }
}

#[command]
pub fn on_log(message: &str) {
    log(message);
}

#[command]
pub fn on_log_error(context: &str, message: &str) {
    log_error(context, message);
}

#[command]
pub fn on_test() -> String {
    println!("on_test");
    println!();
    "ping".to_string()
}
