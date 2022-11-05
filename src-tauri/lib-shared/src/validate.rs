// Types ──────────────────────────────────────────────────────────────────── //

/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// Invalid
///
/// Properties:
///
/// 	message	String		@ pub
///
/// ── End Def ─────────────────

#[derive(Debug, PartialEq, Clone)]
pub struct Invalid {
    pub details: String,
}

impl Invalid {
    pub fn new(details: &str) -> Self {
        Invalid {
            details: details.to_string(),
        }
    }
}

// Validations ────────────────────────────────────────────────────────────── //

pub fn check(valid: bool, details: &str) -> Option<Invalid> {
    if !valid {
        return Some(Invalid::new(details));
    }
    None
}

pub fn check_not_empty(value: &str) -> Option<Invalid> {
    check(value.is_empty(), "The value is empty.")
}

pub fn check_not_blank(value: &str) -> Option<Invalid> {
    check(value.trim().is_empty(), "The value is blank.")
}
