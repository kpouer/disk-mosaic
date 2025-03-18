use egui::Color32;
use rand::Rng;
use std::path::Path;
use treemap::{Mappable, Rect};

#[derive(Debug, Default)]
pub struct Data {
    pub path: String,
    size: u64,
    pub bounds: treemap::Rect,
    pub color: Color32,
    pub children: Vec<Data>,
    pub kind: Kind,
}

#[derive(Default, Debug, PartialEq)]
pub enum Kind {
    #[default]
    Dir,
    File,
}

impl Data {
    pub fn new_directory(name: String) -> Self {
        let mut rnd = rand::rng();
        Self {
            path: name,
            kind: Kind::Dir,
            color: Color32::from_rgb(rnd.random::<u8>(), rnd.random::<u8>(), rnd.random::<u8>()),
            ..Default::default()
        }
    }

    pub fn new_file(path: &Path) -> Self {
        let file_size = path.metadata().unwrap().len();
        let mut rnd = rand::rng();
        Self {
            path: path.to_string_lossy().to_string(),
            kind: Kind::File,
            size: file_size,
            color: Color32::from_rgb(rnd.random::<u8>(), rnd.random::<u8>(), rnd.random::<u8>()),
            ..Default::default()
        }
    }

    pub fn compute_size(&mut self) -> u64 {
        if self.kind == Kind::Dir {
            self.size = self
                .children
                .iter_mut()
                .fold(0, |acc, child| acc + child.compute_size());
        }
        self.size
    }
}

impl Mappable for Data {
    fn size(&self) -> f64 {
        self.size as f64
    }

    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds
    }
}
