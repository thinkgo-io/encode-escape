use float_eq::float_eq;
use serde::{Deserialize, Serialize};

/// Created with CodeCrank
///
/// ── Crank Def ───────────────
///
/// Window
///
/// Options: serialize, builder
///
/// Properties:
///
/// 	monitor	    Option:String		@ pub	# Monitor Name
/// 	x	        f64		            @ pub
/// 	y	        f64		            @ pub
/// 	width	    f64		            @ pub
/// 	height	    f64		            @ pub
/// 	maximized	bool		        @ pub
///
/// ── End Def ─────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    pub monitor: Option<String>,
    pub x: f64, // Left Side
    pub y: f64, // Top Side
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

    pub fn builder() -> WindowBuilder {
        WindowBuilder::default()
    }

    pub fn into_builder(&self) -> WindowBuilder {
        WindowBuilder::from(self)
    }
}

pub struct WindowBuilder {
    pub monitor: Option<String>,
    pub x: f64, // Left Side
    pub y: f64, // Top Side
    pub width: f64,
    pub height: f64,
    pub maximized: bool,
}

impl WindowBuilder {
    pub fn default() -> Self {
        WindowBuilder {
            monitor: None,
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            maximized: false,
        }
    }

    pub fn build(self) -> Window {
        Window {
            monitor: self.monitor,
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            maximized: self.maximized,
        }
    }

    pub fn from(original: &Window) -> Self {
        WindowBuilder {
            monitor: original.monitor.clone(),
            x: original.x,
            y: original.y,
            width: original.width,
            height: original.height,
            maximized: original.maximized,
        }
    }

    pub fn monitor(mut self, monitor: Option<String>) -> Self {
        self.monitor = monitor;
        self
    }

    pub fn x(mut self, x: f64) -> Self {
        self.x = x;
        self
    }

    pub fn y(mut self, y: f64) -> Self {
        self.y = y;
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    pub fn maximized(mut self, maximized: bool) -> Self {
        self.maximized = maximized;
        self
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
