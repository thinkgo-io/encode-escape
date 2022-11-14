use tauri::Window;

use crate::data::app_info::APP_NAME;
use crate::log::*;
use crate::settings::types::RuntimeSettings;

pub fn log_encoding(settings: &RuntimeSettings, input: &str) {
    log_lines(vec![
        "On Encoding:",
        &format!("  Encoding:  {}", settings.encoding),
        &format!("  Operation: {}", settings.operation),
        &format!("  Input:     {}", input),
    ]);
}

pub fn set_window_title(window: &Window, settings: &RuntimeSettings) {
    window.set_title(&to_title(settings)).unwrap();
}

pub fn to_title(settings: &RuntimeSettings) -> String {
    format!(
        "{} {} - {}",
        settings.operation, settings.encoding, APP_NAME
    )
}
