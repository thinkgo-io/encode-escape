use crate::settings::types::RuntimeSettings;
use crate::utils::tauri::*;
use shared::prelude::*;
use shared::settings::SettingsFile;
use tauri::Window;

// Public ─────────────────────────────────────────────── //

/// Errors:
///
/// Not trapping or returning errors from here as failure should not affect the program in any way.
/// We simply wouldn't be able to restore old runtime settings.

pub fn read_runtime_settings(file: &SettingsFile) -> Option<RuntimeSettings> {
    if file.not_exists() {
        return None;
    }
    match file.read() {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(settings) => Some(settings),
            Err(error) => {
                log_and_return("Couldn't deserialize runtime settings", Error::from(error))
            }
        },
        Err(error) => log_and_return("Couldn't read runtime settings file", error),
    }
}

pub fn update_runtime_settings_window(window: &Window, settings: &mut RuntimeSettings) {
    if window.is_fullscreen().unwrap_or(false) {
        return;
    }

    if window.is_maximized().unwrap_or(false) {
        settings.window.maximized = true;
        return;
    }

    update_window_settings(window, &mut settings.window)
}

pub fn write_runtime_settings(settings: &RuntimeSettings, file: &SettingsFile) {
    match serde_json::to_string(settings) {
        Ok(content) => match file.write(&content) {
            Ok(_) => (),
            Err(error) => log_error("Couldn't write runtime settings file", error),
        },
        Err(error) => log_error("Couldn't serialize runtime settings", Error::from(error)),
    }
}

// Private ────────────────────────────────────────────── //

fn log_error(message: &str, error: Error) {
    println!("Error: {} - {}", message, error);
}

fn log_and_return(message: &str, error: Error) -> Option<RuntimeSettings> {
    log_error(message, error);
    None
}

fn update_window_settings(window: &Window, setting: &mut app::types::Window) {
    setting.maximized = false;

    if let Ok(position) = window_to_logical_position(window) {
        setting.x = position.x;
        setting.y = position.y;
    }

    if let Ok(size) = window_to_logical_size(window) {
        setting.width = size.width;
        setting.height = size.height;
    }
}
