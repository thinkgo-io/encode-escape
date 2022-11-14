use shared::prelude::*;

/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// EncodeOperation
///
/// Properties:
///
/// 	encoding	String		@ pub
/// 	operation	String		@ pub
///
///
/// ── End Def ─────────────────

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct EncodeOperation {
    pub encoding: String,
    pub operation: String,
}

impl EncodeOperation {
    pub fn new(encoding: &str, operation: &str) -> Self {
        EncodeOperation {
            encoding: encoding.to_string(),
            operation: operation.to_string(),
        }
    }
}
