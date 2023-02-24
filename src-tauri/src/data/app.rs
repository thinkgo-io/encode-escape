// pub const DEFAULT_LOG_FILE: &str = "encode-escape.log";

#[cfg(target_os = "macos")]
pub const APP_SETTINGS_DIR: &str = "io.thinkgo.encode-escape";
#[cfg(not(target_os = "macos"))]
pub const APP_SETTINGS_DIR: &str = "encode-escape";
pub const APP_RUNTIME_SETTINGS_FILE: &str = "settings.yaml";
