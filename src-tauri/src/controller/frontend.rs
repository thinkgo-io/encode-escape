use std::ops::DerefMut;
use tauri::command;
use tauri::Result;
use tauri::State;
use tauri::Window;

use encode::encode;

use crate::controller::utils::*;
use crate::data::encoding_data::get_encodings;
use crate::log::*;
use crate::settings::types::WrappedRuntimeSettings;
use crate::types::Encoding;

// Commands ───────────────────────────────────────────────────────────────── //

#[command]
pub fn on_get_encodings() -> Vec<Encoding> {
    log("Get Encodings");
    get_encodings()
}

#[command]
pub fn on_encode(
    encoding: &str,
    operation: &str,
    input: &str,
    settings: State<'_, WrappedRuntimeSettings>,
    window: Window,
) -> String {
    let mut guard = settings.lock().unwrap();
    let settings = guard.deref_mut();

    settings.encoding(encoding).operation(operation);
    set_window_title(&window, settings);

    log_encoding(settings, input);

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
