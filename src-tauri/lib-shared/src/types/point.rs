use serde::{Deserialize, Serialize};

/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// Point
///
/// Options: serialize
///
/// Properties:
///
/// 	x	T		@ pub
/// 	y	T		@ pub
///
///
/// ── End Def ─────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}
