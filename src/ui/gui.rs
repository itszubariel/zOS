use crate::ui::graphics;
use alloc::vec::Vec;

pub struct Window {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub title: &'static str,
}

pub struct TerminalWindow {
    pub window: Window,
    pub buffer: Vec<u8>,
}

impl TerminalWindow {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        TerminalWindow {
            window: Window { x, y, width, height, title: "Terminal" },
            buffer: Vec::new(),
        }
    }

    pub fn render(&self) {
        // Draw window background
        graphics::draw_rect(
            self.window.x,
            self.window.y,
            self.window.width,
            self.window.height,
            0x222222
        );
        // Draw window title bar
        graphics::draw_rect(self.window.x, self.window.y, self.window.width, 20, 0x555555);
    }
}
