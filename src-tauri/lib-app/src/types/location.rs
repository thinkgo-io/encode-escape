/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// Location
/// 
/// Options: constructor
/// 
/// Properties:
/// 
/// 	x	i32		@ pub
/// 	y	i32		@ pub
/// 
///
/// ── End Def ─────────────────

#[derive(Clone, Debug)]
pub struct Location {
	pub x: i32,
	pub y: i32,
}

impl Location {

	pub fn new(x: i32, y: i32) -> Self {
		Location {x, y}
	}
}