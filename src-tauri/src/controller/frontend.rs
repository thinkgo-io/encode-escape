use std::ops::Deref;
use std::ops::DerefMut;
use tauri::command;
use tauri::State;
use tauri::Window;

use encode::encode;
use encode::types::EncodeOperation;

use crate::controller::utils::*;
use crate::domain::encode::data::get_encodings;
use crate::domain::encode::encoding::*;
use crate::domain::encode::types::Encoding;
use crate::log::*;
use crate::system::settings::types::WrappedRuntimeSettings;

// Commands ───────────────────────────────────────────────────────────────── //

#[command]
pub fn on_get_encodings() -> Vec<Encoding> {
    get_encodings()
}

#[command]
pub fn on_get_current_encode_operation(settings: State<WrappedRuntimeSettings>) -> EncodeOperation {
    let guard = settings.lock().unwrap();
    let settings = guard.deref();
    settings.encode_operation.clone()
}

#[command]
pub fn on_encode(
    encode_operation: EncodeOperation,
    input: &str,
    settings: State<'_, WrappedRuntimeSettings>,
    window: Window,
) -> String {
    let mut guard = settings.lock().unwrap();
    let settings = guard.deref_mut();
    let encode_operation = normalize_encode_operation(&encode_operation);

    settings.encode_operation(encode_operation);
    set_window_title(&window, settings);

    log_encoding(settings, input);

    match encode(&settings.encode_operation, input) {
        Ok(output) => output,
        Err(error) => ["[Not Encodable]\n\n", &error.to_string()].concat(),
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
