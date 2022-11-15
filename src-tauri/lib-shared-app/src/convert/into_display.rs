use crate::types::Display;
use crate::utils::tauri::*;
use shared::prelude::*;
use tauri::Monitor;
use tauri::Window;

impl From<Monitor> for Display {
    fn from(monitor: Monitor) -> Self {
        let position = monitor_to_logical_position(&monitor);
        let size = monitor_to_logical_size(&monitor);
        Display {
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
        }
    }
}

impl TryFrom<&Window> for Display {
    type Error = Error;
    fn try_from(window: &Window) -> Result<Self> {
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
