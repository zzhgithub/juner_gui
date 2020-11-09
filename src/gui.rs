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

// 顶层的GUI 抽象
#[derive(Clone)]
pub struct Gui {
    //高
    height: u32,
    //宽
    width: u32,
    // 层级
    level: usize,
    // 位置 (x,y)
    position: (u32, u32),
    // 这里还有一套 子组件 ？？考虑是否要名字？
    childern: Vec<Box<Gui>>,
    //todo 事件绑定？
}

trait DisplayUi {
    // 绘图方法
    fn draw(&self) -> !;
}

// 顶层GUI容器
pub struct GuiLayer<D>
where
    D: DrawTarget<Rgb888>,
{
    //高
    height: u32,
    //宽
    width: u32,
    graphic: D,
    // root_gui: Gui,
}

impl<D> GuiLayer<D>
where
    D: DrawTarget<Rgb888>,
{
    pub fn new(graphic: D) -> Self {
        GuiLayer {
            width: graphic.size().width,
            height: graphic.size().height,
            graphic: graphic,
            // todo 这里要设置最上级的 gui
        }
    }

    // 要有一个draw方法？
    
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
