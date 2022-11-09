use crate::types::Display;
use tauri::Monitor;
impl From<&Monitor> for Display {
    fn from(monitor: &Monitor) -> Self {
        let factor = monitor.scale_factor();
        let position = monitor.position().to_logical::<f64>(factor);
        let size = monitor.size().to_logical::<f64>(factor);
        Display {
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
        }
    }
}
