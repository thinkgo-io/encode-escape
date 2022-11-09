use serde::{Deserialize, Serialize};

/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// Size
///
/// Options: serialize
///
/// Properties:
///
/// 	width	T		@ pub
/// 	height	T		@ pub
///
///
/// ── End Def ─────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Size { width, height }
    }
}
