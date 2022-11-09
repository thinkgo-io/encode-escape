use crate::data::app::*;
use crate::settings::types::AppSettings;
use shared::os::types::OSSettings;

pub fn new_app_settings(settings: &OSSettings) -> AppSettings {
    AppSettings::new(
        settings.join(&settings.user_settings_dir, APP_SETTINGS_DIR),
        APP_RUNTIME_SETTINGS_FILE,
    )
}
