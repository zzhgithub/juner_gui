use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
    time::Duration,
};

use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use juner_gui::{component::mouse::Mouse, gui::GuiLayer, DrawTarget, Pixel, Rgb888, Size};

fn main() {
    let display = SimulatorDisplay::<Rgb888>::new(Size::new(800, 600));
    let display = Arc::new(Mutex::new(display));

    let mut gui_layer = GuiLayer::new(DisplayWrapper(display.clone()));
    let mut mouse = Box::new(Mouse::default());
    gui_layer.add_component(mouse);
    gui_layer.draw();
    // gui_layer.test_draw();

    // 显示
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Example", &output_settings);
    loop {
        window.update(&display.lock().unwrap());
        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

struct DisplayWrapper(Arc<Mutex<SimulatorDisplay<Rgb888>>>);

impl DrawTarget<Rgb888> for DisplayWrapper {
    type Error = Infallible;

    fn draw_pixel(&mut self, item: Pixel<Rgb888>) -> Result<(), Self::Error> {
        self.0.lock().unwrap().draw_pixel(item)
    }

    fn size(&self) -> Size {
        self.0.lock().unwrap().size()
    }
}
