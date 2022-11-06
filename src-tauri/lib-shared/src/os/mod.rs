pub mod constants;
pub mod linux;
pub mod macos;
pub mod types;
pub mod windows;

pub use types::OSType;
pub use types::OS;

pub fn get_os() -> OS {
    match consts::OS {
        "linux" => OS::linux.get_os(),
        "macos" => OS::macos.get_os(),
        "windows" => OS::windows.get_so(),
        _ => OS::linux.get_os(),
    }
}
