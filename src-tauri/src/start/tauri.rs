use tauri::App;
use tauri::Manager;

use shared::prelude::*;
use shared_app::convert::into_error::from_tauri_error;
use shared_app::utils::tauri::*;

use crate::controller::utils::*;
use crate::system::configure::types::*;
use crate::system::settings::defaults::*;

pub fn setup_tauri(app: &mut App, configuration: ArcedConfiguration) -> StandardBoxedResultOk {
    setup_window(app, &configuration)?;

    app.manage(configuration.os_settings);
    app.manage(configuration.app_settings);
    app.manage(configuration.runtime_settings);
    app.manage(configuration.runtime_settings_file);

    Ok(())
}

pub fn setup_window(app: &mut App, configuration: &ArcedConfiguration) -> StandardBoxedResultOk {
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
