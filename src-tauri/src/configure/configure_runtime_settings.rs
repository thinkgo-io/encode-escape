use crate::settings::defaults::*;
use crate::settings::types::AppSettings;
use crate::settings::types::RuntimeSettings;
use crate::settings::utils::runtime_settings::read_runtime_settings;
use shared::settings::SettingsFile;

// Public ─────────────────────────────────────────────── //

/// Create the runtime settings for the application.
/// Will not fail (returns default if any issues).
pub fn new_runtime_settings(file: &SettingsFile) -> RuntimeSettings {
    let file_settings = read_runtime_settings(file);

    match file_settings {
        Some(settings) => settings,
        None => default_settings(),
    }
}

pub fn new_runtime_settings_file(app_settings: &AppSettings) -> SettingsFile {
    SettingsFile::new(
        &app_settings.settings_dir,
        &app_settings.runtime_settings_file,
    )
}
