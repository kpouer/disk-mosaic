use std::path::Path;
use crate::data::Data;

pub(crate) struct Task {
    path: String,
}

impl Task {
    pub(crate) fn run(&self) -> Data {
        let mut data = Data::new(self.path.clone());
        let path = Path::new(&self.path);
        if let Ok(iter) = path.read_dir() {
            for path in iter
                .flatten()
                .map(|p| p.path()) {
                if path.is_dir() {
                    data.children.push(Task::new(path.to_string_lossy().to_string()).run());
                } else {
                    let file_size = path.metadata().unwrap().len();
                    data.value += file_size as f64;
                    data.children.push(Data::new_with_value(path.to_string_lossy().to_string(), file_size as f64));
                }
            }
        } else {
            println!("Error reading directory: {path:?}");
        }
        data
    }
}

impl Task {
    pub(crate) fn new(path: String) -> Self {
        Self { path }
    }
}