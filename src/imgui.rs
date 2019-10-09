extern crate imgui_winit_support;

use crate::application::Application;
use crate::events;
use crate::layer::Layer;
use crate::winit_support::WinitWindow;

pub struct ImGuiLayer {
    name: String,
    imgui_context: imgui::Context,
}

impl ImGuiLayer {
    pub fn new(name: &str) -> Self {
        let mut imgui_context = imgui::Context::create();
        // configure imgui-rs Context if necessary

        Self {
            name: String::from(name),
            imgui_context,
        }
    }

    pub fn with_winit_window(&mut self, winit_window: &winit::Window) {
        
    }
}

impl Layer for ImGuiLayer {
    fn on_attach(&self) {}

    fn on_detach(&self) {}
    fn on_update(&self) {
        //info!("ImGuiLayer::Update")
    }
    fn on_event(&self, event: &events::Event) -> bool {

        false
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}
