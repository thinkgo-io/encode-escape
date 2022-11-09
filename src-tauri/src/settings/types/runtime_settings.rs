use app::types::Window;
use serde::{Deserialize, Serialize};

/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// RuntimeSettings
///
/// Properties:
///
/// 	encoding	Option:String	@ pub
/// 	operation	Option:String	@ pub
/// 	window		Option:Window	@ pub
///
///
/// ── End Def ─────────────────

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RuntimeSettings {
    pub encoding: String,
    pub operation: String,
    pub window: Window,
}

impl RuntimeSettings {
    pub fn new(encoding: &str, operation: &str, window: Window) -> Self {
        RuntimeSettings {
            encoding: encoding.to_string(),
            operation: operation.to_string(),
            window,
        }
    }
}
