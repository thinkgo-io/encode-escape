use serde::{Deserialize, Serialize};

use crate::types::Operation;

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
