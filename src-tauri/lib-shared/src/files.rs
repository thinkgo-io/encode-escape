use std::fs::metadata;
use std::fs::remove_file;
use std::path::Path;
use std::time::SystemTime;

use crate::error::Result;
use crate::error::ResultOk;

// Types ──────────────────────────────────────────────────────────────────── //

pub struct FileMonitor {
    pub file_name: String,
    pub modified: Option<SystemTime>,
}

impl FileMonitor {
    pub fn new(file_name: &str) -> Self {
        FileMonitor {
            file_name: file_name.to_string(),
            modified: None,
        }
    }

    pub fn read(file_name: &str) -> Self {
        FileMonitor {
            file_name: file_name.to_string(),
            modified: file_modified(file_name),
        }
    }

    pub fn exists(&self) -> bool {
        file_exists(&self.file_name)
    }

    pub fn is_changed(&self) -> bool {
        let modified = file_modified(&self.file_name);
        modified != self.modified
    }

    pub fn not_exists(&self) -> bool {
        file_not_exists(&self.file_name)
    }

    pub fn update(&mut self) {
        self.modified = file_modified(&self.file_name);
    }
}

// Functions ──────────────────────────────────────────────────────────────── //

/// Deletes a file.
/// Only deletes if the file exists.
pub fn delete(file_name: &str) -> ResultOk {
    if file_exists(file_name) {
        remove_file(file_name)?;
    }
    Ok(())
}

pub fn file_exists(file_name: &str) -> bool {
    Path::new(file_name).exists()
}

pub fn file_not_exists(file_name: &str) -> bool {
    !file_exists(file_name)
}

/// Returns the modified time of a file.
/// If not present or error, returns None.
pub fn file_modified(file_name: &str) -> Option<SystemTime> {
    let metadata = metadata(file_name);
    if metadata.is_err() {
        return None;
    }

    match metadata.unwrap().modified() {
        Ok(time) => Some(time),
        Err(_) => None,
    }
}

pub fn no_file_exists(file_name: &str) -> bool {
    !file_exists(file_name)
}

pub fn read_string(path: &str) -> Result<String> {
    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}

pub fn write_string(path: &str, contents: String) -> ResultOk {
    Ok(std::fs::write(path, contents.as_bytes())?)
}
