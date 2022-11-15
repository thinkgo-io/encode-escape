use crate::prelude::*;
use crate::utils::tauri::*;
use app::types::Display;
use shared::prelude::*;
use tauri::Monitor;
use tauri::Window;

impl From<Wrap<&Monitor>> for Display {
    fn from(monitor: Wrap<&Monitor>) -> Self {
        let monitor = monitor.0;
        let position = monitor_to_logical_position(monitor);
        let size = monitor_to_logical_size(monitor);
        Display {
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
        }
    }
}

impl TryFrom<Wrap<&Window>> for Display {
    type Error = Error;
    fn try_from(window: Wrap<&Window>) -> Result<Self> {
        let window = window.0;
        let position = window_to_logical_position(window)?;
        let size = window_to_logical_size(window)?;
        Ok(Display {
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
        })
    }
}
