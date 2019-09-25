use crate::event;
use crate::layer::{Layer, LayerStack};
use crate::window::{Window, WindowProps};
use std::rc::Rc;

pub struct Application {
    running: bool,
    layer_stack: LayerStack,
    // TODO: Make this a unique pointer?
    window: Window,
    window_ed: event::Dispatcher,
}

impl Application {
    pub fn new() -> Application {
        Application {
            layer_stack: LayerStack::new(),
            window: Window::new(&WindowProps::defaults()),
            window_ed: event::Dispatcher::new(),
            running: false,
        }
    }

    pub fn run(&mut self) {
        self.running = true;

        info!("Starting the Reaper Engine...");

        while self.running {
            self.layer_stack.update();
            self.window.on_update(&mut self.window_ed);

            self.window_ed.read_all().iter().for_each(|event| {
                self.on_event(event);
            });
        }

        info!("Closing the Reaper Engine...")
    }

    pub fn on_event(&mut self, event: &event::Event) {
        let handled = match event.get_event_type() {
            event::EventType::WindowClose => self.on_window_close(&event),
            _ => false,
        };

        if !handled {
            self.layer_stack.on_event(event);
        }
    }

    pub fn push_layer(&mut self, layer: Rc<dyn Layer>) {
        self.layer_stack.push_layer(layer);
    }

    pub fn push_overlay(&mut self, overlay: Rc<dyn Layer>) {
        self.layer_stack.push_overlay(overlay);
    }

    fn on_window_close(&mut self, _event: &event::Event) -> bool {
        self.running = false;
        true
    }
}
