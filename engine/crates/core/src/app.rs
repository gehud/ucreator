pub trait Layer {
    fn on_create(&mut self, app: &mut App);

    fn on_update(&mut self, app: &mut App);
}

pub struct App {
    is_runing: bool,
    layers: Vec<Box<dyn Layer>>
}

impl App {
    pub fn new() -> Self {
        Self {
            is_runing: false,
            layers: Vec::new()
        }
    }

    pub fn add_layer(&mut self, layer: impl Layer + 'static) -> &mut Self {
        let mut handle = Box::new(layer);
        handle.on_create(self);
        self.layers.push(handle);
        self
    }

    pub fn run(&mut self) {
        self.is_runing = true;

        while self.is_runing {
            for i in 0..self.layers.len() {
                let layer = self.layers[i].as_mut() as *mut dyn Layer;
                unsafe {
                    layer.as_mut().unwrap().on_update(self);
                }
            }
        }
    }

    pub fn stop(&mut self) {
        self.is_runing = false;
    }
}
