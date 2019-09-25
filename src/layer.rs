use crate::event;
use std::rc::Rc;

pub trait Layer {
    fn on_attach(&self);
    fn on_detach(&self);
    fn on_update(&self);
    fn on_event(&self, event: &event::Event);

    fn get_name(&self) -> &str;
}

pub struct LayerStack {
    layers: Vec<Rc<dyn Layer>>,
    layer_insert: usize,
}

impl LayerStack {
    pub fn new() -> Self {
        Self {
            layer_insert: 0,
            layers: Vec::new(),
        }
    }

    pub fn update(&self) {
        for layer in self.layers.iter() {
            layer.on_update();
        }
    }

    pub fn on_event(&self, event: &event::Event) {
        for layer in self.layers.iter().rev() {
            layer.on_event(event);
            if event.handled {
                break;
            }
        }
    }

    pub fn push_layer<'a>(&mut self, layer: Rc<dyn Layer>) {
        self.layers.insert(self.layer_insert, layer);
        self.layer_insert += 1;
    }

    pub fn push_overlay(&mut self, overlay: Rc<dyn Layer>) {
        self.layers.push(overlay);
    }

    pub fn pop_layer(&mut self, layer: Rc<dyn Layer>) {
        let index = self
            .layers
            .iter()
            .position(|l| Rc::ptr_eq(l, &layer))
            .unwrap();
        if index < self.layer_insert {
            self.layers.remove(index);
            self.layer_insert -= 1;
        }
    }

    pub fn pop_overlay(&mut self, layer: Rc<dyn Layer>) {
        let index = self
            .layers
            .iter()
            .position(|l| Rc::ptr_eq(l, &layer))
            .unwrap();
        if index >= self.layer_insert {
            self.layers.remove(index);
        }
    }
}
