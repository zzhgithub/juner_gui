use embedded_graphics::{drawable::Pixel, pixelcolor::Rgb888, prelude::Point};

use crate::gui::DisplayUi;
pub struct Whiteboard {
    pub height: u32,
    pub width: u32,
    pub level: usize,
    pub position: (u32, u32),
    pub color: Rgb888,
    pub locked: bool,
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
            locked: false,
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
        unimplemented!()
    }

    // 刷新缓存的方法 然一个统一的地方的缓存刷新
    fn flush(&mut self) {
        use crate::gui::DISPLAY_BUFFER;
        let (x, y) = self.position;
        if !self.locked {
            for i in x..self.height {
                for j in y..self.width {
                    // 画全部的点
                    DISPLAY_BUFFER
                        .write()
                        .push(Pixel(Point::new(i as i32, j as i32), self.color));
                    // 这里的内存怎么不溢出
                }
            }
            self.locked = true;
        }
    }
}
