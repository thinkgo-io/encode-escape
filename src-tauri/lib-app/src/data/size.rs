/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// Size
///
/// Options: constructor
///
/// Properties:
///
/// 	width	i32		@ pub
/// 	height	i32		@ pub
///
///
/// ── End Def ─────────────────

#[derive(Clone, Debug)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    pub fn new(width: i32, height: i32) -> Self {
        Size { width, height }
    }
}
