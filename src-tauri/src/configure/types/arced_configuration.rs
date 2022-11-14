use shared::os::types::OSSettings;
use shared::settings::SettingsFile;
use std::sync::Arc;
use std::sync::Mutex;

use crate::configure::types::Configuration;
use crate::settings::types::*;
use crate::utils::sync::*;

/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// Configuration
///
/// Properties:
///
/// 	os_settings			Arc:Mutex:OSSettings			@ pub
/// 	app_settings		Arc:Mutex:AppSettings			@ pub
/// 	runtime_settings	Arc:Mutex:RuntimeSettings		@ pub
/// 	runtime_file		Arc:Mutex:SettingsFile		    @ pub
///
///
/// ── End Def ─────────────────

pub struct ArcedConfiguration {
    pub os_settings: Arc<OSSettings>,
    pub app_settings: Arc<AppSettings>,
    pub runtime_settings: Arc<Mutex<RuntimeSettings>>,
    pub runtime_settings_file: Arc<SettingsFile>,
}

impl ArcedConfiguration {
    pub fn new(
        os_settings: OSSettings,
        app_settings: AppSettings,
        runtime_settings: RuntimeSettings,
        runtime_settings_file: SettingsFile,
    ) -> Self {
        ArcedConfiguration {
            os_settings: into_arc(os_settings),
            app_settings: into_arc(app_settings),
            runtime_settings: into_arc_mutex(runtime_settings),
            runtime_settings_file: into_arc(runtime_settings_file),
        }
    }

    pub fn clone(&self) -> ArcedConfiguration {
        ArcedConfiguration {
            os_settings: self.os_settings.clone(),
            app_settings: self.app_settings.clone(),
            runtime_settings: self.runtime_settings.clone(),
            runtime_settings_file: self.runtime_settings_file.clone(),
        }
    }
}

impl From<Configuration> for ArcedConfiguration {
    fn from(configuration: Configuration) -> Self {
        ArcedConfiguration::new(
            configuration.os_settings,
            configuration.app_settings,
            configuration.runtime_settings,
            configuration.runtime_settings_file,
        )
    }
}
