use embedded_graphics::pixelcolor::Rgb888;

use crate::gui::DisplayUi;
pub struct Whiteboard {
    pub height: u32,
    pub width: u32,
    pub level: usize,
    pub position: (u32, u32),
    pub color: Rgb888,
}

impl Whiteboard {
    // 创建一个白板
    pub fn new(height: u32, width: u32, level: usize, position: (u32, u32), color: Rgb888) -> Self {
        Whiteboard {
            height,
            width,
            level,
            position,
            color,
        }
    }
}

impl DisplayUi for Whiteboard {
    fn level(&mut self, level: usize) {
        self.level = level;
    }

    fn position(&mut self, position: (u32, u32)) {
        self.position = position;
    }

    fn resize(&mut self, height: u32, width: u32) {
        self.height = height;
        self.width = width;
    }

    fn draw(&mut self) {
       for i in (0..self.width) {
           for j in (0..self.height){
                //
           }
       }
    }
}
