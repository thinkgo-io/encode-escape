use serde::{Deserialize, Serialize};

/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// Operation
///
/// Properties:
///
/// 	name	&str		@ pub static
/// 	reverse	&str		@ pub static
/// 	label	&str		@ pub static
/// 	description	&str		@ pub static
///
///
/// ── End Def ─────────────────

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub name: &'static str,
    pub reverse: &'static str,
    pub label: &'static str,
    pub description: &'static str,
}

impl Operation {
    pub fn new(
        name: &'static str,
        reverse: &'static str,
        label: &'static str,
        description: &'static str,
    ) -> Self {
        Operation {
            name,
            reverse,
            label,
            description,
        }
    }
}
