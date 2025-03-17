use std::path::Path;
use crate::data::Data;

pub(crate) struct Task {
    path: String,
}

impl Task {
    pub(crate) fn new(path: String) -> Self {
        Self { path }
    }

    pub(crate) fn run(&self) -> Data {
        let mut data = Data::new_directory(self.path.clone());
        let path = Path::new(&self.path);
        if let Ok(iter) = path.read_dir() {
            for path in iter
                .flatten()
                .map(|p| p.path()) {
                if path.is_dir() {
                    let data_dir = Task::new(path.to_string_lossy().to_string()).run();
                    data.children.push(data_dir);
                } else {
                    let data_file = Data::new_file(&path);
                    data.children.push(data_file);
                }
            }
        } else {
            println!("Error reading directory: {path:?}");
        }
        data.compute_size();
        data
    }
}