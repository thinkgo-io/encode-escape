use crate::prelude::*;
use crate::utils::tauri::*;
use app::types::Window;
use shared::prelude::*;

impl TryFrom<Wrap<&tauri::Window>> for Window {
    type Error = Error;
    fn try_from(window: Wrap<&tauri::Window>) -> Result<Self> {
        let window = window.0;
        let position = window_to_logical_position(window)?;
        let size = window_to_logical_size(window)?;
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
