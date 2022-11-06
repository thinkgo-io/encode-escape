use os::OS;
use os::OSType;
use os::linux::constants::*;
use expand::*;

pub fn get_os() -> Result<OS> {

    let home = environment(USER_APP_SETTINGS_SUBDIR);
    let settings = home.to_string() + FILE_SEPARATOR + USER_APP_SETTINGS_SUBDIR;

    OS::Builder::new()
        .os(OSType::MacOS)
        .newline(NEWLINE)
        .file_separator(FILE_SEPARATOR)
        .path_separator(PATH_SEPARATOR)
        .user_home_dir(home)
        .user_settings_dir(settings)
        .build()
}