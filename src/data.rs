use egui::{Color32, ImageSource, include_image};
use log::warn;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use treemap::{Mappable, Rect};

#[derive(Debug, Default)]
pub struct Data {
    depth: u16,
    pub name: String,
    pub size: u64,
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

impl Kind {
    pub fn get_image(&self) -> ImageSource {
        match self {
            Kind::Dir => include_image!("../assets/directory.svg"),
            Kind::File => include_image!("../assets/file.svg"),
        }
    }
}

static INDEX: AtomicUsize = AtomicUsize::new(0);

impl Data {
    pub fn new_directory(path: &Path, depth: u16) -> Self {
        Self {
            depth,
            name: Self::get_file_name(path),
            kind: Kind::Dir,
            color: Self::next_color(),
            ..Default::default()
        }
    }

    pub fn new_file(path: &Path, depth: u16) -> Self {
        let size = path.metadata().map(|metadata| metadata.len()).unwrap_or(0);
        Self {
            depth,
            name: Self::get_file_name(path),
            kind: Kind::File,
            size,
            color: Self::next_color(),
            ..Default::default()
        }
    }

    fn get_file_name(path: &Path) -> String {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "?".to_string());
        name
    }

    fn next_color() -> Color32 {
        let idx = INDEX
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
                Some((v + 1) % egui_solarized::ACCENT_COLORS.len())
            })
            .unwrap_or_else(|e| {
                warn!("AtomicUsize error: {}", e);
                egui_solarized::ACCENT_COLORS.len()
            });
        egui_solarized::ACCENT_COLORS[idx]
    }

    pub(crate) fn push(&mut self, child: Data) {
        self.children.push(child);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn compute_size(&mut self) -> u64 {
        if self.kind == Kind::Dir {
            self.size = self.children.iter().fold(0, |acc, x| acc + x.size);
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
