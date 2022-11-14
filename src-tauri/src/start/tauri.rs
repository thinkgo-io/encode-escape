use tauri::generate_handler;
use tauri::App;
use tauri::Builder;
use tauri::LogicalPosition;
use tauri::LogicalSize;
use tauri::Manager;
use tauri::Window;

use app::types::Window as WindowSetting;
use app::utils::window::*;

use crate::configure::types::*;
use crate::controller::events::*;
use crate::controller::frontend::*;
use crate::convert::into_error::*;
use crate::prelude::*;
use crate::settings::defaults::*;
use crate::settings::types::RuntimeSettings;
use crate::utils::tauri::*;
use shared::prelude::*;

// Main ───────────────────────────────────────────────── //

pub fn start(configuration: &ArcedConfiguration) -> StandardBoxedResultOk {
    let configuration_1 = configuration.clone();
    let configuration_2 = configuration.clone();

    Builder::default()
        .setup(|app| setup_tauri(app, configuration_1))
        .invoke_handler(generate_handler![
            on_get_encodings,
            on_encode,
            on_log,
            on_log_error,
            on_set_title,
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
    let settings = get_runtime_settings(configuration);

    window
        .set_min_size(Some(min_window_size()))
        .map_err(tauri_error)?;
    let settings = normalize_settings_position(settings, &window);
    position_window(&settings.window, &mut window)?;

    set_runtime_settings(configuration, settings);

    Ok(())
}

// Window Functions ───────────────────────────────────── //

fn get_runtime_settings(configuration: &ArcedConfiguration) -> RuntimeSettings {
    configuration.runtime_settings.lock().unwrap().clone()
}

fn is_default_window(window: &WindowSetting) -> bool {
    window == &default_window()
}

fn normalize_settings_position(settings: RuntimeSettings, window: &Window) -> RuntimeSettings {
    let monitor = get_monitor(window);
    if let None = monitor {
        return settings;
    }
    let display = Wrap(&(monitor.unwrap())).into();

    let updated_window = match is_default_window(&settings.window) {
        true => center(&settings.window, &display),
        false => normalize_position(&display, &settings.window, default_window),
    };
    settings.window(updated_window)
}

fn position_window(settings: &WindowSetting, window: &mut Window) -> StandardBoxedResultOk {
    window.set_size(LogicalSize::new(settings.width, settings.height))?;
    window.set_position(LogicalPosition::new(settings.x, settings.y))?;
    Ok(())
}

fn set_runtime_settings(configuration: &ArcedConfiguration, settings: RuntimeSettings) {
    *configuration.runtime_settings.lock().unwrap() = settings;
}
