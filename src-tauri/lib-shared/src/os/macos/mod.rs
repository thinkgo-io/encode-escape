pub mod constants;

use crate::prelude::*;

use crate::environment::*;
use crate::os::macos::constants::*;
use crate::os::types::*;

pub fn get_os() -> Result<OSSettings> {
    let home = environment(USER_APP_SETTINGS_SUBDIR)?;
    let settings = [&home, FILE_SEPARATOR, USER_APP_SETTINGS_SUBDIR].concat();

    Ok(OSSettings {
        os: OS::Mac,
        newline: NEWLINE.to_string(),
        file_separator: FILE_SEPARATOR.to_string(),
        path_separator: PATH_SEPARATOR.to_string(),
        user_home_dir: home,
        user_settings_dir: settings,
    })
}
