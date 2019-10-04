#[macro_use]
extern crate log;

mod application;
mod event;
mod layer;
mod logger;
mod window;
mod imgui;

use crate::layer::Layer;
use crate::imgui::ImGuiLayer;
use crate::window::{Window, WindowProps, WinitWindow};

struct ExampleLayer {
    name: String,
}

impl Layer for ExampleLayer {
    fn on_attach(&self) {}
    fn on_detach(&self) {}
    fn on_update(&self) {
        //info!("ExampleLayer::Update")
    }
    fn on_event(&self, event: &event::Event) -> bool {
        trace!("{:?}", event);
        false
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

fn main() {
    logger::init();
    let window = Box::new(WinitWindow::new(&WindowProps::defaults()));
    let mut app = application::Application::new(window);

    let imgui_layer = ImGuiLayer::new("ImGui Layer", &app);
    // imgui_layer.with_winit_window(app.get_window());
    let example_layer = ExampleLayer { name: String::from("Example Layer") };

    app.push_layer(Box::new(example_layer));
    app.push_layer(Box::new(imgui_layer));
    app.run();
}
