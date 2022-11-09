use crate::types::Display;
use crate::types::Window;

pub fn center(window: &Window, display: &Display) -> Window {
    let x = display.x + (display.width - window.width) / 2.0;
    let y = display.y + (display.height - window.height) / 2.0;
    Window::new(None, x, y, window.width, window.height, false)
}

pub fn fits_inside(window: &Window, display: &Display) -> bool {
    window.width <= display.width && window.height <= display.height
}

pub fn is_inside(window: &Window, display: &Display) -> bool {
    window.x >= display.x
        && window.y >= display.y
        && window.x + window.width <= display.x + display.width
        && window.y + window.height <= display.y + display.height
}
