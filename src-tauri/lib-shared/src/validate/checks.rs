use crate::validate::types::Invalid;

// Validations ────────────────────────────────────────────────────────────── //

pub fn check(valid: bool, details: &str) -> Option<Invalid> {
    if !valid {
        return Some(Invalid::new(details));
    }
    None
}

pub fn check_not_blank(value: &str) -> Option<Invalid> {
    check(value.trim().is_empty(), "The value must not be blank.")
}

pub fn check_not_blank_option(value: Option<&str>) -> Option<Invalid> {
    match value {
        Some(_) => check_not_blank(value.unwrap()),
        None => non_is_invalid(),
    }
}

pub fn check_not_empty(value: &str) -> Option<Invalid> {
    check(value.is_empty(), "The value must not be empty.")
}

pub fn check_not_empty_option(value: Option<&str>) -> Option<Invalid> {
    match value {
        Some(_) => check_not_empty(value.unwrap()),
        None => non_is_invalid(),
    }
}

pub fn check_not_none<T>(value: Option<T>) -> Option<Invalid> {
    match value {
        Some(_) => None,
        None => non_is_invalid(),
    }
}

// Utility functions ──────────────────────────────────────────────────────── //

fn non_is_invalid() -> Option<Invalid> {
    Some(Invalid::new("The value can not be None."))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_returns_none_when_valid() {
        assert_eq!(None, check(true, "details"));
    }

    #[test]
    fn check_returns_invalid_when_not_valid() {
        assert_eq!(Some(Invalid::new("details")), check(false, "details"));
    }
}
