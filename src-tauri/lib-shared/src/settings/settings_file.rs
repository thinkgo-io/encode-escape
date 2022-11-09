use crate::prelude::*;
use crate::utils::files::file_exists;
use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::write;
use std::path::MAIN_SEPARATOR;

#[derive(Debug, PartialEq, Clone)]
pub struct SettingsFile {
    pub directory: String,
    pub file: String,
}

impl SettingsFile {
    pub fn new(directory: &str, file: &str) -> Self {
        SettingsFile {
            directory: directory.to_string(),
            file: file.to_string(),
        }
    }

    pub fn exists(&self) -> bool {
        file_exists(&self.path())
    }

    pub fn not_exists(&self) -> bool {
        !self.exists()
    }

    pub fn read(&self) -> Result<String> {
        Ok(read_to_string(self.path())?)
    }

    pub fn write(&self, value: &String) -> Result<()> {
        create_dir_all(&self.directory)?;
        write(&self.path(), value.as_bytes())?;
        Ok(())
    }

    pub fn path(&self) -> String {
        format!("{}{}{}", self.directory, MAIN_SEPARATOR, self.file)
    }
}
