use crate::component::whiteboard::Whiteboard;
use alloc::boxed::Box;
use alloc::vec::Vec;
use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::Rgb888,
    prelude::Pixel,
    prelude::*,
    primitives::Circle,
    style::{PrimitiveStyle, TextStyle},
};
use lazy_static::lazy_static;
use spin::RwLock;

lazy_static! {
    pub static ref DISPLAY_BUFFER: RwLock<Vec<Pixel<Rgb888>>> = RwLock::new(Vec::new());
}

pub trait DisplayUi {
    // 重新设置 等级
    fn level(&mut self, level: usize);
    // 重新定义位置
    fn position(&mut self, position: (u32, u32));
    // 重新定义
    fn resize(&mut self, height: u32, width: u32);
    // 绘图方法 不建议使用这个方法
    fn draw(&mut self);
    // 刷新缓存的方法
    fn flush(&mut self);
    // 获取当前等级
    fn get_level(&mut self) -> &usize;
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
                Rgb888::new(0, 0, 0),
            ))],
        }
    }

    // 要有一个draw方法？
    pub fn draw(&mut self) {
        // 清空数据
        DISPLAY_BUFFER.write().clear();
        // 先排序

        // let c_list = &self.components;
        // c_list.sort_by(|a: Box<dyn DisplayUi>, b: Box<dyn DisplayUi>| {
        //     a.get_level().cmp(&b.get_level())
        // });

        for component in self.components.iter_mut() {
            // todo 这里的刷选没有每个都管理自己的buffer
            component.flush();
        }
        // 画全部的像素点
        self.graphic
            .draw_iter(DISPLAY_BUFFER.read().iter().copied());
    }

    // 添加一个组件
    pub fn add_component(&mut self, component: Box<dyn DisplayUi>) {
        self.components.push(component);
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
