use crate::data::runtime::*;
use crate::settings::types::AppSettings;
use crate::settings::types::RuntimeSettings;
use crate::settings::utils::read_runtime_settings;
use app::types::Display;
use app::types::Window;
use app::window::utils::*;
use shared::settings::SettingsFile;
use tauri::Monitor;

// Public ─────────────────────────────────────────────── //

/// Create the runtime settings for the application.
/// Will not fail (returns default if any issues).
pub fn new_runtime_settings(file: &SettingsFile, monitor: &Monitor) -> RuntimeSettings {
    match read_runtime_settings(file) {
        Some(settings) => normalize(settings, monitor),
        None => default_runtime_settings(monitor),
    }
}

pub fn new_runtime_settings_file(app_settings: &AppSettings) -> SettingsFile {
    SettingsFile::new(
        &app_settings.settings_dir,
        &app_settings.runtime_settings_file,
    )
}

// Private ────────────────────────────────────────────── //

fn default_runtime_settings(monitor: &Monitor) -> RuntimeSettings {
    RuntimeSettings::new(
        DEFAULT_ENCODING,
        DEFAULT_OPERATION,
        center(&default_window(), &(Display::from(monitor))),
    )
}

fn default_window() -> Window {
    Window::new(None, 0.0, 0.0, DEFAULT_WIDTH, DEFAULT_HEIGHT, false)
}

fn normalize(settings: RuntimeSettings, monitor: &Monitor) -> RuntimeSettings {
    RuntimeSettings::new(
        &settings.encoding,
        &settings.operation,
        normalize_window(&settings.window, monitor),
    )
}

fn normalize_window(window: &Window, monitor: &Monitor) -> Window {
    let display = Display::from(monitor);

    if is_inside(&window, &display) {
        return window.to_owned();
    } else if fits_inside(&window, &display) {
        return center(window, &display);
    } else {
        return center(&default_window(), &display);
    }
}
