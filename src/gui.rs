use alloc::boxed::Box;
use alloc::vec::Vec;
use embedded_graphics::{
    fonts::{Font6x8, Text},
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::Circle,
    style::{PrimitiveStyle, TextStyle},
};

use crate::component::whiteboard::Whiteboard;

pub trait DisplayUi {
    // 重新设置 等级
    fn level(&mut self, level: usize);
    // 重新定义位置
    fn position(&mut self, position: (u32, u32));
    // 重新定义
    fn resize(&mut self, height: u32, width: u32);
    // 绘图方法
    fn draw(&mut self);
}

// 顶层GUI容器
pub struct GuiLayer<D>
where
    D: DrawTarget<Rgb888>,
{
    //高
    pub height: u32,
    //宽
    pub width: u32,
    pub graphic: D,
    pub components: Vec<Box<dyn DisplayUi>>,
}

impl<D> GuiLayer<D>
where
    D: DrawTarget<Rgb888>,
{
    pub fn new(graphic: D) -> Self {
        let mut size = graphic.size();
        GuiLayer {
            height: size.height,
            width: size.width,
            graphic: graphic,
            components: vec![Box::new(Whiteboard::new(
                size.width,
                size.height,
                0,
                (0, 0),
                Rgb888::new(255, 255, 255),
            ))],
        }
    }

    // 要有一个draw方法？
    pub fn draw(&mut self) {
        for component in self.components.iter_mut() {
            component.draw();
        }
    }

    // 测试画图方法
    pub fn test_draw(&mut self) {
        let c =
            Circle::new(Point::new(20, 20), 8).into_styled(PrimitiveStyle::with_fill(Rgb888::RED));
        let t = Text::new("Hello Rust!", Point::new(20, 16))
            .into_styled(TextStyle::new(Font6x8, Rgb888::GREEN));
        c.draw(&mut self.graphic);
        t.draw(&mut self.graphic);
    }
}
