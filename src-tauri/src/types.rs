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
/// 	variants	Vec:Variant		@ pub, static
///
///
/// ── End Def ─────────────────

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Encoding {
    pub name: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub variants: Vec<Variant>,
}

impl Encoding {
    pub fn new(
        name: &'static str,
        label: &'static str,
        description: &'static str,
        variants: Vec<Variant>,
    ) -> Self {
        Encoding {
            name,
            label,
            description,
            variants,
        }
    }
}

/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// Variant
///
/// Properties:
///
/// 	name	&str		@ pub static
/// 	label	&str		@ pub static
/// 	description	&str		@ pub static
///
///
/// ── End Def ─────────────────

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Variant {
    pub name: &'static str,
    pub reverse: &'static str,
    pub label: &'static str,
    pub description: &'static str,
}

impl Variant {
    pub fn new(
        name: &'static str,
        reverse: &'static str,
        label: &'static str,
        description: &'static str,
    ) -> Self {
        Variant {
            name,
            reverse,
            label,
            description,
        }
    }
}
