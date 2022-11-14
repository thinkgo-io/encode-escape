use shared::prelude::*;
use tauri::App;
use tauri::LogicalPosition;
use tauri::LogicalSize;
use tauri::Manager;
use tauri::Monitor;
use tauri::Window;

pub fn get_main_window(app: &App) -> Result<Window> {
    match app.get_window("main") {
        Some(window) => Ok(window),
        None => Err(Error::invalid_configuration(
            "Can't get Tauri window 'main'.",
        )),
    }
}

pub fn get_monitor(window: &Window) -> Option<Monitor> {
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

pub fn window_to_logical_position(window: &Window) -> Result<LogicalPosition<f64>> {
    let factor = window.scale_factor().unwrap_or(1.0);
    Ok(window
        .outer_position()
        .map_err(to_error)?
        .to_logical::<f64>(factor))
}

pub fn window_to_logical_size(window: &Window) -> Result<LogicalSize<f64>> {
    let factor = window.scale_factor().unwrap_or(1.0);
    Ok(window
        .outer_size()
        .map_err(to_error)?
        .to_logical::<f64>(factor))
}

fn to_error(error: tauri::Error) -> Error {
    Error::error(Box::new(error), "Tauri Error")
}
