use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::GlobalWindowEvent;
use tauri::Window;
use tauri::WindowEvent;

use shared::settings::SettingsFile;

use crate::settings::types::RuntimeSettings;
use crate::settings::utils::*;

type WrappedSettings = Arc<Mutex<RuntimeSettings>>;

pub fn on_window_event(event: GlobalWindowEvent, settings: &WrappedSettings, file: &SettingsFile) {
    let window = event.window();

    match event.event() {
        WindowEvent::CloseRequested { .. } => {
            on_close_window(&window, &settings, &file);
        }
        WindowEvent::Moved(_) => {
            on_move_window(&window, &settings);
        }
        WindowEvent::Resized(_) => {
            on_resize_window(&window, &settings);
        }
        _ => (),
    }
}

fn on_close_window(window: &Window, settings: &WrappedSettings, settings_file: &SettingsFile) {
    let mut guard = settings.lock().unwrap();
    let settings = guard.deref_mut();

    write_runtime_settings(settings, &settings_file);
}

fn on_move_window(window: &Window, settings: &WrappedSettings) {
    let mut guard = settings.lock().unwrap();
    let settings = guard.deref_mut();

    update_runtime_settings_window(&window, settings);
}

fn on_resize_window(window: &Window, settings: &WrappedSettings) {
    let mut guard = settings.lock().unwrap();
    let settings = guard.deref_mut();

    update_runtime_settings_window(&window, settings);
}
