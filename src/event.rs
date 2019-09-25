use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub struct Event {
    pub handled: bool,
    event_type: EventType,
}

impl Event {
    pub fn new(event_type: EventType) -> Self {
        Self {
            handled: false,
            event_type,
        }
    }

    pub fn get_event_type(&self) -> &EventType {
        &self.event_type
    }
}

#[derive(Debug)]
pub enum EventType {
    None,

    WindowClose,
    WindowResize { width: f64, height: f64 },
    WindowFocus,
    WindowLostFocus,
    WindowMoved { window_x: f64, window_y: f64 },

    AppTick,
    AppUpdate,
    AppRender,

    KeyPressed { key_code: u32, repeat_count: u32 },
    KeyReleased { key_code: u32 },

    MouseButtonPressed { button: u32 },
    MouseButtonReleased { button: u32 },
    MouseMoved { mouse_x: f64, mouse_y: f64 },
    MouseScrolled { scroll: u32 },
}

pub struct Dispatcher {
    sender: Sender<Event>,
    receiver: Receiver<Event>,
}

impl Dispatcher {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            sender: tx,
            receiver: rx,
        }
    }

    pub fn dispatch(&mut self, event: Event) {
        self.sender.send(event).expect("Unable to Dispatch event!");
    }

    pub fn read_all(&mut self) -> Vec<Event> {
        self.receiver.try_iter().collect()
    }

    pub fn read_one(&mut self) -> Event {
        self.receiver.try_recv().unwrap()
    }
}
