use crate::system::settings::types::RuntimeSettings;
use encode::types::EncodeOperation;
use shared_app::types::Window;
use tauri::LogicalSize;
use tauri::Size;

pub const MINIMUM_WIDTH: f64 = 496.0;
pub const MINIMUM_HEIGHT: f64 = 292.0;
pub const DEFAULT_WIDTH: f64 = 800.0;
pub const DEFAULT_HEIGHT: f64 = 600.0;
pub const DEFAULT_X: f64 = 100.0;
pub const DEFAULT_Y: f64 = 100.0;

pub const DEFAULT_ENCODING: &str = "base64";
pub const DEFAULT_OPERATION: &str = "encode";

pub fn default_encode_operation() -> EncodeOperation {
    EncodeOperation::new(DEFAULT_ENCODING, DEFAULT_OPERATION)
}

pub fn default_settings() -> RuntimeSettings {
    RuntimeSettings::new(default_encode_operation(), default_window())
}

pub fn default_window() -> Window {
    Window::new(
        None,
        DEFAULT_X,
        DEFAULT_Y,
        DEFAULT_WIDTH,
        DEFAULT_HEIGHT,
        false,
    )
}

pub fn min_window_size() -> Size {
    Size::Logical(LogicalSize {
        width: MINIMUM_WIDTH,
        height: MINIMUM_HEIGHT,
    })
}
