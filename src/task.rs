use crate::data::Data;
use std::path::Path;
use std::sync::mpsc::Sender;

use std::path::PathBuf;

pub struct Task {
    path: PathBuf,
    tx: Sender<Data>,
}

impl Task {
    pub fn new(path: PathBuf, tx: Sender<Data>) -> Self {
        Self { path, tx }
    }

    pub fn run(&self) {
        let mut data = Data::new_directory(self.path.to_string_lossy().to_string());
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut waiting = Self::scan_directory(&self.path, &sender);

        while waiting > 0 {
            if let Ok(d) = receiver.recv() {
                waiting -= 1;
                data.children.push(d);
            }
        }
        data.compute_size();
        self.tx.send(data).unwrap();
    }

    pub fn scan_directory(path: &Path, sender: &Sender<Data>) -> usize {
        let mut waiting = 0;
        if let Ok(iter) = path.read_dir() {
            for path in iter.flatten().map(|p| p.path()) {
                waiting += 1;
                if path.is_dir() {
                    let sender = sender.clone();
                    Task::new(path, sender).run();
                } else {
                    sender.send(Data::new_file(&path)).unwrap();
                }
            }
        } else {
            println!("Error reading directory: {path:?}");
        }
        waiting
    }
}
