use os::OS;
use os::OSType;
use os::windows::constants::*;
use expand::*;

pub fn get_os() -> Result<OS> {

    let home = environment(USER_APP_SETTINGS_SUBDIR);
    let settings = environment(USER_APP_SETTINGS_DIR_VARIABLE);

    OS::Builder::new()
        .os(OSType::Windows)
        .newline(NEWLINE)
        .file_separator(FILE_SEPARATOR)
        .path_separator(PATH_SEPARATOR)
        .user_home_dir(home)
        .user_settings_dir(settings)
        .build()
}