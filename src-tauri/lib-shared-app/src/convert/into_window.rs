use crate::types::Window;
use crate::utils::tauri::*;
use shared::prelude::*;
use tauri::Window as TauriWindow;

impl TryFrom<TauriWindow> for Window {
    type Error = Error;
    fn try_from(window: TauriWindow) -> Result<Self> {
        let position = window_to_logical_position_offset(&window)?;
        let size = window_to_logical_size(&window)?;
        Ok(Window {
            monitor: None,
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
            maximized: false,
        })
    }
}
