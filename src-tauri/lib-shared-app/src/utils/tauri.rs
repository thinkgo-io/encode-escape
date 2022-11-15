use crate::types::Window as SettingsWindow;
use crate::utils::window::*;
use shared::prelude::*;
use tauri::App;
use tauri::LogicalPosition;
use tauri::LogicalSize;
use tauri::Manager;
use tauri::Monitor;
use tauri::Window as TauriWindow;

// Public ─────────────────────────────────────────────────────────────────── //

pub fn get_main_window(app: &App) -> Result<TauriWindow> {
    match app.get_window("main") {
        Some(window) => Ok(window),
        None => Err(Error::invalid_configuration(
            "Can't get Tauri window 'main'.",
        )),
    }
}

pub fn get_monitor(window: &TauriWindow) -> Option<Monitor> {
    window.current_monitor().unwrap_or(None)
}

pub fn monitor_to_logical_position(monitor: &Monitor) -> LogicalPosition<f64> {
    let factor = monitor.scale_factor();
    monitor.position().to_logical::<f64>(factor)
}

pub fn monitor_to_logical_size(monitor: &Monitor) -> LogicalSize<f64> {
    let factor = monitor.scale_factor();
    monitor.size().to_logical::<f64>(factor)
}

pub fn move_tauri_window(
    settings: &SettingsWindow,
    tauri_window: &mut TauriWindow,
) -> StandardBoxedResultOk {
    tauri_window.set_size(LogicalSize::new(settings.width, settings.height))?;
    tauri_window.set_position(LogicalPosition::new(settings.x, settings.y))?;
    Ok(())
}

pub fn normalize_settings_window_position(
    settings_window: &SettingsWindow,
    tauri_window: &TauriWindow,
    default: &SettingsWindow,
) -> SettingsWindow {
    let monitor = get_monitor(tauri_window);

    if let None = monitor {
        return settings_window.clone();
    }
    let display = monitor.unwrap().into();

    if settings_window == default {
        return center(&settings_window, &display);
    }

    let settings_window = window_offset_to_absolute_position(&settings_window, &tauri_window);
    normalize_position(&display, &settings_window, &default)
}

pub fn update_settings_window(tauri_window: &TauriWindow, settings_window: &mut SettingsWindow) {
    if tauri_window.is_fullscreen().unwrap_or(false) {
        return;
    }

    if tauri_window.is_maximized().unwrap_or(false) {
        settings_window.maximized = true;
        return;
    }

    settings_window.maximized = false;
    update_settings_window_position(tauri_window, settings_window);
    update_settings_window_size(tauri_window, settings_window);
}

pub fn update_settings_window_position(
    tauri_window: &TauriWindow,
    settings_window: &mut SettingsWindow,
) {
    if let Ok(position) = window_to_logical_position_offset(tauri_window) {
        settings_window.x = position.x;
        settings_window.y = position.y;
    }
}

pub fn update_settings_window_size(
    tauri_window: &TauriWindow,
    settings_window: &mut SettingsWindow,
) {
    if let Ok(size) = window_to_logical_size(tauri_window) {
        settings_window.width = size.width;
        settings_window.height = size.height;
    }
}

pub fn window_offset_to_absolute_position(
    settings_window: &SettingsWindow,
    tauri_window: &TauriWindow,
) -> SettingsWindow {
    if let Some(monitor) = get_monitor(tauri_window) {
        let monitor_position = monitor_to_logical_position(&monitor);
        return settings_window
            .into_builder()
            .x(settings_window.x + monitor_position.x)
            .y(settings_window.y + monitor_position.y)
            .build();
    }
    settings_window.clone()
}

pub fn window_to_logical_position(window: &TauriWindow) -> Result<LogicalPosition<f64>> {
    let factor = window.scale_factor().unwrap_or(1.0);
    Ok(window
        .outer_position()
        .map_err(to_error)?
        .to_logical::<f64>(factor))
}

pub fn window_to_logical_position_offset(window: &TauriWindow) -> Result<LogicalPosition<f64>> {
    let position = window_to_logical_position(window)?;
    match get_monitor(window) {
        Some(monitor) => {
            let monitor_position = monitor_to_logical_position(&monitor);
            Ok(LogicalPosition::new(
                position.x - monitor_position.x,
                position.y - monitor_position.y,
            ))
        }
        None => Ok(position),
    }
}

pub fn window_to_logical_size(window: &TauriWindow) -> Result<LogicalSize<f64>> {
    let factor = window.scale_factor().unwrap_or(1.0);
    Ok(window
        .outer_size()
        .map_err(to_error)?
        .to_logical::<f64>(factor))
}

// Private ────────────────────────────────────────────────────────────────── //

fn to_error(error: tauri::Error) -> Error {
    Error::error(Box::new(error), "Tauri Error")
}
