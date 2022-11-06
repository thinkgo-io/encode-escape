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