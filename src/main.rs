#[macro_use]
extern crate log;

mod application;
mod event;
mod layer;
mod logger;
mod window;
mod imgui;

use crate::layer::Layer;
use std::rc::Rc;
use crate::imgui::ImGuiLayer;

struct ExampleLayer {
    name: String,
}

impl Layer for ExampleLayer {
    fn on_attach(&self) {}
    fn on_detach(&self) {}
    fn on_update(&self) {
        //info!("ExampleLayer::Update")
    }
    fn on_event(&self, event: &event::Event) {
        trace!("{:?}", event)
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

fn main() {
    logger::init();
    let mut app = application::Application::new();
    app.push_layer(Rc::new(ExampleLayer {
        name: String::from("Example Layer"),
    }));
    app.push_layer(Rc::new(ImGuiLayer::new("ImGui Layer")));
    app.run();
}
