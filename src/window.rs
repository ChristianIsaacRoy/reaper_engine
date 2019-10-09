use crate::events;

pub struct WindowProps {
    pub title: String,
    pub width: f64,
    pub height: f64,
}

impl WindowProps {
    pub fn defaults() -> Self {
        WindowProps {
            title: String::from("Reaper Engine"),
            width: 1280.0,
            height: 720.0,
        }
    }
}

pub trait Window {
    fn on_update(&mut self, ed: &mut events::EventDispatcher);
}
