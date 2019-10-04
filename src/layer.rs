use crate::event;

pub trait Layer {
    fn on_attach(&self);
    fn on_detach(&self);
    fn on_update(&self);
    fn on_event(&self, event: &event::Event) -> bool;

    fn get_name(&self) -> &str;
}

pub struct LayerStack {
    layers: Vec<Box<dyn Layer>>,
    layer_insert: usize,
}

impl LayerStack {
    pub fn new() -> Self {
        Self {
            layer_insert: 0,
            layers: Vec::new(),
        }
    }

    pub fn on_update(&self) {
        for layer in self.layers.iter() {
            layer.on_update();
        }
    }

    pub fn on_event(&self, event: &event::Event) -> bool {
        let mut handled = false;
        for layer in self.layers.iter().rev() {
            handled = layer.on_event(event);
            if handled {
                break;
            }
        }
        handled
    }

    pub fn push_layer<'a>(&mut self, layer: Box<dyn Layer>) -> usize {
        self.layers.insert(self.layer_insert, layer);
        self.layer_insert += 1;
        self.layer_insert
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) -> usize {
        self.layers.push(overlay);
        self.layers.len()
    }

    pub fn pop_layer(&mut self, index: usize) -> Option<Box<dyn Layer>> {
        if self.layers.len() > 0 && index < self.layer_insert {
            self.layer_insert -= 1;
            Option::Some(self.layers.remove(index))
        }
        else {
            Option::None
        }
    }

    pub fn pop_overlay(&mut self, index: usize) -> Option<Box<dyn Layer>> {
        if self.layers.len() > 0 && index >= self.layer_insert  && index < self.layers.len() {
            Option::Some(self.layers.remove(index))
        }
        else {
            Option::None
        }
    }
}
