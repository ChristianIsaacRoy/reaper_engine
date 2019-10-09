use crate::events;
use crate::layer::{Layer, LayerStack};
use crate::window::{Window, WindowProps};

pub struct Application {
    running: bool,
    layer_stack: LayerStack,
    pub window: Box<dyn Window>, // Box because window is a trait ( we don't know how big the implementation of the Window trait is )
    window_ed: events::EventDispatcher,
}

impl Application {
    pub fn new(window: Box<dyn Window>) -> Application {
        Application {
            layer_stack: LayerStack::new(),
            window,
            window_ed: events::EventDispatcher::new(),
            running: false,
        }
    }

    pub fn run(&mut self) {
        self.running = true;

        info!("Starting the Reaper Engine...");

        while self.running {
            self.layer_stack.on_update();
            self.window.on_update(&mut self.window_ed);

            self.window_ed.read_all().iter().for_each(|event| {
                self.on_event(event);
            });
        }

        info!("Closing the Reaper Engine...")
    }

    pub fn on_event(&mut self, event: &events::Event) {
        match event.get_event_type() {
            events::EventType::WindowClose => self.on_window_close(&event),
            _ => (),
        };

        self.layer_stack.on_event(event);
    }

    fn on_window_close(&mut self, _event: &events::Event) {
        self.running = false
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layer_stack.push_layer(layer);
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layer_stack.push_overlay(overlay);
    }

    pub fn get_window(&self) -> &Box<dyn Window> {
        &self.window
    }
}
