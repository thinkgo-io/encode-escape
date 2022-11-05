use tauri::command;
use tauri::Result;
use tauri::Window;

use encode::encode;

use crate::encoding::get_encodings;
use crate::encoding::Encoding;
use crate::log::*;

#[command]
pub fn on_get_encodings() -> Vec<Encoding> {
    log("Get Encodings");
    get_encodings()
}

#[command]
pub fn on_encode(encoding: &str, operation: &str, input: &str) -> String {
    log_lines(vec![
        "On Encoding:",
        &format!("  Encoding: {}", encoding),
        &format!("  Operation:  {}", operation),
        &format!("  Input:    {}", input),
    ]);

    match encode(&encoding.to_lowercase(), &operation.to_lowercase(), input) {
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
pub fn on_set_title(window: Window, title: String) -> Result<()> {
    window.set_title(&title)
}

#[command]
pub fn on_test() -> String {
    println!("on_test");
    println!();
    "ping".to_string()
}
