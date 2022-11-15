#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::generate_handler;
use tauri::App;
use tauri::Builder;
use tauri::Manager;

use shared::prelude::*;
use shared_app::convert::into_error::from_tauri_error;
use shared_app::utils::tauri::*;

use crate::controller::events::*;
use crate::controller::frontend::*;
use crate::controller::utils::*;
use crate::system::configure::types::*;
use crate::system::settings::defaults::*;

// Main ───────────────────────────────────────────────── //

pub fn start(configuration: &ArcedConfiguration) -> StandardBoxedResultOk {
    let configuration_1 = configuration.clone();
    let configuration_2 = configuration.clone();

    Builder::default()
        .setup(|app| setup_tauri(app, configuration_1))
        .invoke_handler(generate_handler![
            on_get_encodings,
            on_get_current_encode_operation,
            on_encode,
            on_log,
            on_log_error,
            on_test
        ])
        .on_window_event(move |event| {
            on_window_event(
                event,
                &configuration_2.runtime_settings,
                &configuration_2.runtime_settings_file,
            )
        })
        .run(tauri::generate_context!())
        .expect("Could not launch Encode Escape");

    Ok(())
}

fn setup_tauri(app: &mut App, configuration: ArcedConfiguration) -> StandardBoxedResultOk {
    setup_window(app, &configuration)?;

    app.manage(configuration.os_settings);
    app.manage(configuration.app_settings);
    app.manage(configuration.runtime_settings);
    app.manage(configuration.runtime_settings_file);

    Ok(())
}

fn setup_window(app: &mut App, configuration: &ArcedConfiguration) -> StandardBoxedResultOk {
    let mut window = get_main_window(app)?;
    let mut settings = configuration.runtime_settings.lock().unwrap();
    settings.window =
        normalize_settings_window_position(&settings.window, &window, &default_window());

    window
        .set_min_size(Some(min_window_size()))
        .map_err(from_tauri_error)?;

    move_tauri_window(&settings.window, &mut window)?;
    set_window_title(&window, &settings);

    window.show()?;
    Ok(())
}
