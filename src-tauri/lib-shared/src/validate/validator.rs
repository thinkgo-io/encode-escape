use crate::prelude::*;
use crate::validate::Invalid;

/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// Validator
///
/// Options: default
///
/// Properties:
///
/// 	errors	Vec:Invalid
///
/// ── End Def ─────────────────

#[derive(Debug, PartialEq, Clone)]
pub struct Validator {
    pub errors: Vec<Invalid>,
}

impl Validator {
    pub fn new(errors: Vec<Invalid>) -> Self {
        Validator { errors }
    }

    pub fn add(&mut self, error: Invalid) -> bool {
        self.errors.push(error);
        false
    }

    fn add_invalid_none(&mut self, name: &str) -> bool {
        self.add(Invalid::new(
            &self.to_name_details(name, "The value can not be None."),
        ))
    }

    pub fn add_with_details(&mut self, name: &str, details: &str) -> bool {
        self.add(Invalid::new(&self.to_name_details(name, details)))
    }

    pub fn check(&mut self, name: &str, details: &str, valid: bool) -> bool {
        match valid {
            true => true,
            false => self.add(Invalid::new(&[name, " - ", details].concat())),
        }
    }

    pub fn check_not_blank(&mut self, name: &str, value: &str) -> bool {
        self.check(
            name,
            "The value must not be blank.",
            value.trim().is_empty(),
        )
    }

    pub fn check_not_blank_option(&mut self, name: &str, value: Option<&str>) -> bool {
        match value {
            Some(_) => self.check_not_blank(name, value.unwrap()),
            None => self.add_invalid_none(name),
        }
    }

    pub fn check_not_empty(&mut self, name: &str, value: &str) -> bool {
        self.check(name, "The value must not be empty.", value.is_empty())
    }

    pub fn check_not_empty_option(&mut self, name: &str, value: Option<&str>) -> bool {
        match value {
            Some(_) => self.check_not_empty(name, value.unwrap()),
            None => self.add_invalid_none(name),
        }
    }

    pub fn check_not_none<T>(&mut self, name: &str, value: Option<T>) -> bool {
        match value {
            Some(_) => true,
            None => self.add_invalid_none(name),
        }
    }

    pub fn result(&self) -> Result<()> {
        match self.errors.is_empty() {
            true => Ok(()),
            false => Err(Error::invalid_value(&self.join_details())),
        }
    }

    fn join_details(&self) -> String {
        let strings: Vec<String> = self.errors.iter().map(|e| e.details.to_string()).collect();
        strings.join(", ")
    }

    fn to_name_details(&self, name: &str, details: &str) -> String {
        [name, " - ", details].concat()
    }
}

impl Default for Validator {
    fn default() -> Self {
        Validator { errors: Vec::new() }
    }
}
