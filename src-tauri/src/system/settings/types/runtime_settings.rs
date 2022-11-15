use encode::types::EncodeOperation;
use serde::{Deserialize, Serialize};
use shared_app::types::Window;

/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// RuntimeSettings
///
/// Properties:
///
/// 	encode_operation:   EncodeOperation	@ pub
/// 	window		        Option:Window	@ pub
///
/// ── End Def ─────────────────

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RuntimeSettings {
    pub encode_operation: EncodeOperation,
    pub window: Window,
}

impl RuntimeSettings {
    pub fn new(encode_operation: EncodeOperation, window: Window) -> Self {
        RuntimeSettings {
            encode_operation,
            window,
        }
    }

    pub fn encode_operation(&mut self, encode_operation: EncodeOperation) -> &mut Self {
        self.encode_operation = encode_operation;
        self
    }
}
