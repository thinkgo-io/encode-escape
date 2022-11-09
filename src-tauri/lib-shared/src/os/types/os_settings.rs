use crate::os::types::os::OS;

/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// OSSettings
///
/// Properties:
///
/// 	os	OS		@ pub
/// 	newline	String		@ pub
/// 	file_separator	String		@ pub
/// 	path_separator	String		@ pub
/// 	user_home_dir	String		@ pub
/// 	user_settings_dir	String		@ pub
///
///
/// ── End Def ─────────────────

#[derive(Debug, PartialEq, Clone)]
pub struct OSSettings {
    pub os: OS,
    pub newline: String,
    pub file_separator: String,
    pub path_separator: String,
    pub user_home_dir: String,
    pub user_settings_dir: String,
}

impl OSSettings {
    pub fn new(
        os: OS,
        newline: String,
        file_separator: String,
        path_separator: String,
        user_home_dir: String,
        user_settings_dir: String,
    ) -> Self {
        OSSettings {
            os,
            newline,
            file_separator,
            path_separator,
            user_home_dir,
            user_settings_dir,
        }
    }

    pub fn join(&self, dir: &str, subpath: &str) -> String {
        format!("{}{}{}", dir, self.path_separator, subpath)
    }

    pub fn join_vec(&self, paths: Vec<&str>) -> String {
        paths.join(&self.path_separator)
    }
}
