use crate::data::Data;
use std::path::Path;
use std::sync::mpsc::Sender;

pub(crate) struct Task {
    is_root: bool,
    path: String,
    tx: Sender<Data>,
}

impl Task {
    pub(crate) fn new(is_root: bool, path: String, tx: Sender<Data>) -> Self {
        Self { is_root, path, tx }
    }

    pub(crate) fn run(&self) {
        let mut data = Data::new_directory(self.path.clone());
        let path = Path::new(&self.path);
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut waiting = 0;
        let sender = if self.is_root {
            self.tx.clone()
        } else {
            sender.clone()
        };
        if let Ok(iter) = path.read_dir() {
            for path in iter.flatten().map(|p| p.path()) {
                waiting += 1;
                if path.is_dir() {
                    let sender = sender.clone();
                    Task::new(false, path.to_string_lossy().to_string(), sender).run();
                } else {
                    sender.send(Data::new_file(&path)).unwrap();
                }
            }
        } else {
            println!("Error reading directory: {path:?}");
        }

        if !self.is_root {
            while waiting > 0 {
                if let Ok(d) = receiver.recv() {
                    waiting -= 1;
                    data.children.push(d);
                }
            }
            data.compute_size();
            self.tx.send(data).unwrap();
        }
    }
}
