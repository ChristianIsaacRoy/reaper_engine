
extern crate imgui_winit_support;

use crate::application::{Application};
use crate::event;
use crate::layer::{Layer};
use crate::window::WinitWindow;

use imgui_winit_support::{HiDpiMode, WinitPlatform};

pub struct ImGuiLayer {
    name: String,
    imgui_context: imgui::Context,
    imgui_platform: WinitPlatform,
}

impl ImGuiLayer {
    pub fn new(name: &str, app: &Application) -> Self {
        let mut imgui_context = imgui::Context::create();
        let imgui_platform = WinitPlatform::init(&mut imgui_context);
        
        Self {
            name: String::from(name),
            imgui_context,
            imgui_platform,
        }
    }

    pub fn with_winit_window(&mut self, winit_window: &WinitWindow) {
        self.imgui_platform.attach_window(self.imgui_context.io_mut(), winit_window.get_winit_window(), HiDpiMode::Default);
    }
}

impl Layer for ImGuiLayer {
    fn on_attach(&self) {
        
    }

    fn on_detach(&self) {}
    fn on_update(&self) {
        //info!("ImGuiLayer::Update")
    }
    fn on_event(&self, event: &event::Event) -> bool {
        trace!("{:?}", event);
        false
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}