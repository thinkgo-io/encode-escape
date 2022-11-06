use crate::validate::Validator;

/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// OS
///
/// Options: builder
///
/// Properties:
///
/// 	os	OSType		@ pub
/// 	newline	String		@ pub
/// 	file_separator	String		@ pub
/// 	path_separator	String		@ pub
/// 	user_home_dir	String		@ pub
/// 	user_settings_dir	String		@ pub
///
///
/// ── End Def ─────────────────

#[derive(Debug, PartialEq, Clone)]
pub struct OS {
    pub os: OSType,
    pub newline: String,
    pub file_separator: String,
    pub path_separator: String,
    pub user_home_dir: String,
    pub user_settings_dir: String,
}

impl OS {
    pub fn new(
        os: OSType,
        newline: String,
        file_separator: String,
        path_separator: String,
        user_home_dir: String,
        user_settings_dir: String,
    ) -> Self {
        OS {
            os,
            newline,
            file_separator,
            path_separator,
            user_home_dir,
            user_settings_dir,
        }
    }

    pub fn builder() -> OSBuilder {
        OSBuilder::default()
    }

    pub fn to_builder(&self) -> OSBuilder {
        OSBuilder::default().from(self)
    }
}

pub struct OSBuilder {
    pub os: Option<OSType>,
    pub newline: Option<String>,
    pub file_separator: Option<String>,
    pub path_separator: Option<String>,
    pub user_home_dir: Option<String>,
    pub user_settings_dir: Option<String>,
}

impl OSBuilder {
    fn default() -> Self {
        OsBuilder {
            os: None,
            newline: None,
            file_separator: None,
            path_separator: None,
            user_home_dir: None,
            user_doc_dir: None,
            user_settings_dir: None,
        }
    }

    pub fn build(self) -> ResultsIn<Os> {
        self.validate()?;
        Ok(OS {
            os: self.os.unwrap(),
            newline: self.newline.unwrap(),
            file_separator: self.file_separator.unwrap(),
            path_separator: self.path_separator.unwrap(),
            user_home_dir: self.user_home_dir.unwrap(),
            user_settings_dir: self.user_settings_dir.unwrap(),
        })
    }

    pub fn from(mut self, original: &Os) -> Self {
        self.os = original.clone();
        self
    }

    pub fn os(mut self, os: OSType) -> Self {
        self.os.os = os;
        self
    }

    pub fn newline(mut self, newline: String) -> Self {
        self.os.newline = newline;
        self
    }

    pub fn file_separator(mut self, file_separator: String) -> Self {
        self.os.file_separator = file_separator;
        self
    }

    pub fn path_separator(mut self, path_separator: String) -> Self {
        self.os.path_separator = path_separator;
        self
    }

    pub fn user_home_dir(mut self, user_home_dir: String) -> Self {
        self.os.user_home_dir = user_home_dir;
        self
    }

    pub fn user_doc_dir(mut self, user_doc_dir: String) -> Self {
        self.os.user_doc_dir = user_doc_dir;
        self
    }

    pub fn user_settings_dir(mut self, user_settings_dir: String) -> Self {
        self.os.user_settings_dir = user_settings_dir;
        self
    }

    fn validate(&self) -> ResultOk {
        let mut validator = Validator::default();

        validator.check_not_none("OS", self.os);
        validator.check_not_none("newline", self.newline);
        validator.check_not_none("file_separator", self.file_separator);
        validator.check_not_none("path_separator", self.path_separator);
        validator.check_not_none("user_home_dir", self.user_home_dir);
        validator.check_not_none("user_settings_dir", self.user_settings_dir);

        validator.result()
    }
}

pub enum OSType {
    Windows,
    Linux,
    Mac,
}
