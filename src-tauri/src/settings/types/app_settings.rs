/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// AppSettings
///
/// Properties:
///
/// 	settings_dir			String	@ pub
/// 	runtime_settings_file	String	@ pub
///
/// ── End Def ─────────────────

#[derive(Debug, PartialEq, Clone)]
pub struct AppSettings {
    pub settings_dir: String,
    pub runtime_settings_file: String,
}

impl AppSettings {
    pub fn new(settings_dir: String, runtime_settings_file: &str) -> Self {
        AppSettings {
            settings_dir: settings_dir.to_string(),
            runtime_settings_file: runtime_settings_file.to_string(),
        }
    }
}
