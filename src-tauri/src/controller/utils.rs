use tauri::Window;

use crate::data::app_info::APP_NAME;
use crate::domain::encode::encoding::*;
use crate::log::*;
use crate::system::settings::types::RuntimeSettings;

pub fn log_encoding(settings: &RuntimeSettings, input: &str) {
    log_lines(vec![
        "On Encoding:",
        &format!("  Encoding:  {}", settings.encode_operation.encoding),
        &format!("  Operation: {}", settings.encode_operation.operation),
        &format!("  Input:     {}", input),
    ]);
}

pub fn set_window_title(window: &Window, settings: &RuntimeSettings) {
    window.set_title(&to_title(settings)).unwrap();
}

pub fn to_title(settings: &RuntimeSettings) -> String {
    match to_encode_operation_titles(&settings.encode_operation) {
        Some(encode_operation) => {
            if encode_operation.operation.to_lowercase().starts_with("to") {
                return format!(
                    "{} {} - {}",
                    encode_operation.encoding, encode_operation.operation, APP_NAME);
            }
            return format!(
            "{} {} - {}",
            encode_operation.operation, encode_operation.encoding, APP_NAME
        )},
        None => APP_NAME.to_string(),
    }
}
