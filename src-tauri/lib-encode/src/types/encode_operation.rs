use serde::{Deserialize, Serialize};

// Created with CodeCrank
//
// ── Crank Def ───────────────
//
// EncodeOperation
//
// Options: serialize
//
// Properties:
//
// 	encoding	String		@ pub
// 	operation	String		@ pub
//
// ── End Def ─────────────────

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]

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
