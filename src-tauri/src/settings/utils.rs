use crate::settings::types::RuntimeSettings;
use shared::prelude::*;
use shared::settings::SettingsFile;

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

pub fn write_runtime_settings(file: &SettingsFile, settings: &RuntimeSettings) {
    match serde_json::to_string(settings) {
        Ok(content) => match file.write(&content) {
            Ok(_) => (),
            Err(error) => log("Couldn't write runtime settings file", error),
        },
        Err(error) => log("Couldn't serialize runtime settings", Error::from(error)),
    }
}

fn log(message: &str, error: Error) {
    println!("Error: {} - {}", message, error);
}

fn log_and_return(message: &str, error: Error) -> Option<RuntimeSettings> {
    log(message, error);
    None
}
