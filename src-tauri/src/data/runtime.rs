use tauri::LogicalSize;
use tauri::Size;

pub const MINIMUM_WIDTH: f64 = 496.0;
pub const MINIMUM_HEIGHT: f64 = 292.0;
pub const DEFAULT_WIDTH: f64 = 800.0;
pub const DEFAULT_HEIGHT: f64 = 600.0;

pub const DEFAULT_ENCODING: &str = "base64";
pub const DEFAULT_OPERATION: &str = "encode";

pub fn min_window_size() -> Size {
    Size::Logical(LogicalSize {
        width: MINIMUM_WIDTH,
        height: MINIMUM_HEIGHT,
    })
}
