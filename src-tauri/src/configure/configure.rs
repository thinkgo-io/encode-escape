use shared::os::get_os_settings;
use shared::prelude::*;
use tauri::Monitor;

use crate::configure::configure_app_settings::new_app_settings;
use crate::configure::configure_runtime_settings::new_runtime_settings;
use crate::configure::configure_runtime_settings::new_runtime_settings_file;
use crate::configure::types::Configuration;

pub fn create_configuration(monitor: &Monitor) -> Result<Configuration> {
    let os_settings = get_os_settings()?;
    let app_settings = new_app_settings(&os_settings);
    let runtime_settings_file = new_runtime_settings_file(&app_settings);
    let runtime_settings = new_runtime_settings(&runtime_settings_file, monitor);

    p!("OS Settings: {:?}", &os_settings);
    p!("App Settings: {:?}", &app_settings);
    p!("Runtime Settings: {:?}", &runtime_settings);
    p!("Runtime Settings File: {:?}", &runtime_settings_file);

    Ok(Configuration::new(
        os_settings,
        app_settings,
        runtime_settings,
        runtime_settings_file,
    ))
}
