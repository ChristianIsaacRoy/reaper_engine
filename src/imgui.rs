use crate::event;
use crate::layer::{Layer};

pub struct ImGuiLayer {
    name: String,
}

impl ImGuiLayer {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

impl Layer for ImGuiLayer {
    fn on_attach(&self) {

    }

    fn on_detach(&self) {}
    fn on_update(&self) {
        //info!("ImGuiLayer::Update")
    }
    fn on_event(&self, event: &event::Event) {
        trace!("{:?}", event)
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}