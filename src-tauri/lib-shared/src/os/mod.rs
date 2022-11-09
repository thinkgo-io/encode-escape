pub mod linux;
pub mod macos;
pub mod types;
pub mod windows;

use crate::prelude::*;
use std::env::consts;
use types::*;

pub fn get_os_settings() -> Result<OSSettings> {
    match consts::OS {
        "linux" => linux::get_os(),
        "macos" => macos::get_os(),
        "windows" => windows::get_os(),
        _ => linux::get_os(),
    }
}
