pub mod constants;

use crate::prelude::*;

use crate::environment::*;
use crate::os::windows::constants::*;
use crate::os::*;

pub fn get_os() -> Result<OSSettings> {
    let home = environment(USER_HOME_DIR_VARIABLE)?;
    let settings = environment(USER_APP_SETTINGS_DIR_VARIABLE)?;

    Ok(OSSettings {
        os: OS::Windows,
        newline: NEWLINE.to_string(),
        file_separator: FILE_SEPARATOR.to_string(),
        path_separator: PATH_SEPARATOR.to_string(),
        user_home_dir: home,
        user_settings_dir: settings,
    })
}
