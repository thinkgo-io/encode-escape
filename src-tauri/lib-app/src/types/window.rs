use float_eq::float_eq;
use serde::{Deserialize, Serialize};

/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// Window
///
/// Options: serialize
///
/// Properties:
///
/// 	monitor	Option:String		@ pub	# Monitor Name
/// 	x	f64		@ pub
/// 	y	f64		@ pub
/// 	width	f64		@ pub
/// 	height	f64		@ pub
/// 	maximized	bool		@ pub
///
///
/// ── End Def ─────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    pub monitor: Option<String>,
    pub x: f64, // From Left
    pub y: f64, // From Top
    pub width: f64,
    pub height: f64,
    pub maximized: bool,
}

impl Window {
    pub fn new(
        monitor: Option<String>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        maximized: bool,
    ) -> Self {
        Window {
            monitor,
            x,
            y,
            width,
            height,
            maximized,
        }
    }
}

impl PartialEq<Window> for Window {
    fn eq(&self, other: &Window) -> bool {
        self.monitor == other.monitor
            && float_eq!(self.x, other.x, abs <= 0.001)
            && float_eq!(self.y, other.y, abs <= 0.001)
            && float_eq!(self.width, other.width, abs <= 0.001)
            && float_eq!(self.height, other.height, abs <= 0.001)
            && self.maximized == other.maximized
    }
}
