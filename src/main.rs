#[macro_use]
extern crate log;

mod application;
mod events;
mod imgui;
mod layer;
mod logger;
mod window;
mod winit_support;

use crate::imgui::ImGuiLayer;
use crate::layer::Layer;
use crate::window::{Window, WindowProps};
use crate::winit_support::WinitWindow;

struct ExampleLayer {
    name: String,
}

impl Layer for ExampleLayer {
    fn on_attach(&self) {}
    fn on_detach(&self) {}
    fn on_update(&self) {
        //info!("ExampleLayer::Update")
    }
    fn on_event(&self, event: &events::Event) -> bool {
        trace!("{:?}", event);
        false
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

fn main() {
    logger::init();

    let mut window = Box::new(WinitWindow::new(&WindowProps::defaults()));
    
    let mut imgui_layer = ImGuiLayer::new("ImGui Layer");
    imgui_layer.with_winit_window(window.get_winit_window());

    let mut example_layer = ExampleLayer {
        name: String::from("Example Layer"),
    };

    let mut app = application::Application::new(window);

    app.push_layer(Box::new(example_layer));
    app.push_layer(Box::new(imgui_layer));
    app.run();
}
