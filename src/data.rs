use egui::Color32;
use rand::Rng;
use treemap::{Mappable, Rect};

pub struct Data {
    pub(crate) name: String,
    pub(crate) value: f64,
    pub(crate) bounds: treemap::Rect,
    pub(crate) color: Color32,
}

impl Data {
    pub fn new(name: String, value: f64) -> Self {
        let mut rnd = rand::rng();
        Data {
            name,
            value,
            bounds: treemap::Rect::new(),
            color: Color32::from_rgb(
                rnd.random::<u8>(),
                rnd.random::<u8>(),
                rnd.random::<u8>(),
            ),
        }
    }
}

impl Mappable for Data {
    fn size(&self) -> f64 {
        self.value
    }

    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds
    }
}
