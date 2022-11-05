use serde::{Deserialize, Serialize};
/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// Encoding
///
/// Properties:
///
/// 	name	&str		@ pub, static
/// 	label	&str		@ pub, static
/// 	description	&str		@ pub, static
/// 	operations	Vec:Operation		@ pub, static
///
///
/// ── End Def ─────────────────

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Encoding {
    pub name: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub operations: Vec<Operation>,
}

impl Encoding {
    pub fn new(
        name: &'static str,
        label: &'static str,
        description: &'static str,
        operations: Vec<Operation>,
    ) -> Self {
        Encoding {
            name,
            label,
            description,
            operations,
        }
    }
}

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
