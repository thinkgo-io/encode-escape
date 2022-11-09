use crate::settings::types::*;
use shared::os::types::OSSettings;
use shared::settings::SettingsFile;

/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// Configuration
///
/// Properties:
///
/// 	os_settings			OSSettings			@ pub
/// 	app_settings		AppSettings			@ pub
/// 	runtime_settings	RuntimeSettings		@ pub
/// 	runtime_file		SettingsFile		@ pub
///
///
/// ── End Def ─────────────────

#[derive(Debug, PartialEq, Clone)]
pub struct Configuration {
    pub os_settings: OSSettings,
    pub app_settings: AppSettings,
    pub runtime_settings: RuntimeSettings,
    pub runtime_settings_file: SettingsFile,
}

impl Configuration {
    pub fn new(
        os_settings: OSSettings,
        app_settings: AppSettings,
        runtime_settings: RuntimeSettings,
        runtime_settings_file: SettingsFile,
    ) -> Self {
        Configuration {
            os_settings,
            app_settings,
            runtime_settings,
            runtime_settings_file,
        }
    }
}
