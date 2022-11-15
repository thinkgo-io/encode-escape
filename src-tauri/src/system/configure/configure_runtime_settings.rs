use crate::domain::encode::encoding::validate_encode_operation;
use crate::system::settings::defaults::*;
use crate::system::settings::runtime_settings::read_runtime_settings;
use crate::system::settings::types::AppSettings;
use crate::system::settings::types::RuntimeSettings;
use shared::settings::SettingsFile;

// Public ─────────────────────────────────────────────── //

/// Create the runtime settings for the application.
/// Will not fail (returns default if any issues).
pub fn new_runtime_settings(file: &SettingsFile) -> RuntimeSettings {
    match read_runtime_settings(file) {
        Some(settings) => match validate_encode_operation(&settings.encode_operation) {
            Ok(_) => settings,
            Err(_) => RuntimeSettings::new(default_encode_operation(), settings.window),
        },
        None => default_settings(),
    }
}

pub fn new_runtime_settings_file(app_settings: &AppSettings) -> SettingsFile {
    SettingsFile::new(
        &app_settings.settings_dir,
        &app_settings.runtime_settings_file,
    )
}
