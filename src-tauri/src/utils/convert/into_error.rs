use shared::error::Error;

pub fn tauri_error(error: tauri::Error) -> Error {
    Error::error(Box::new(error), "Tauri Error")
}
