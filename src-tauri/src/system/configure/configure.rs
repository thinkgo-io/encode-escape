use shared::os::get_os_settings;
use shared::prelude::*;

use crate::system::configure::configure_app_settings::new_app_settings;
use crate::system::configure::configure_runtime_settings::new_runtime_settings;
use crate::system::configure::configure_runtime_settings::new_runtime_settings_file;
use crate::system::configure::types::Configuration;

pub fn create_configuration() -> Result<Configuration> {
    let os_settings = get_os_settings()?;
    let app_settings = new_app_settings(&os_settings);
    let runtime_settings_file = new_runtime_settings_file(&app_settings);
    let runtime_settings = new_runtime_settings(&runtime_settings_file);

    Ok(Configuration::new(
        os_settings,
        app_settings,
        runtime_settings,
        runtime_settings_file,
    ))
}
