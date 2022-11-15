/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// Display
///
/// Properties:
///
/// 	x	f64		@ pub
/// 	y	f64		@ pub
/// 	width	f64		@ pub
/// 	height	f64		@ pub
///
///
/// ── End Def ─────────────────

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Display {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Display {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Display {
            x,
            y,
            width,
            height,
        }
    }
}
