use serde::{Deserialize, Serialize};

/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// Position
///
/// Options: serialize
///
/// Properties:
///
/// 	x	i32		@ pub
/// 	y	i32		@ pub
///
///
/// ── End Def ─────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}
