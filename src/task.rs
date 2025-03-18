use crate::data::Data;
use std::path::Path;
use std::sync::mpsc::Sender;

pub(crate) struct Task {
    path: String,
    tx: Sender<Data>,
}

impl Task {
    pub(crate) fn new(path: String, tx: Sender<Data>) -> Self {
        Self { path, tx }
    }

    pub(crate) fn run(&self) {
        let mut data = Data::new_directory(self.path.clone());
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

    pub(crate) fn scan_directory(path: &str, sender: &Sender<Data>) -> usize {
        let path = Path::new(path);
        let mut waiting = 0;
        if let Ok(iter) = path.read_dir() {
            for path in iter.flatten().map(|p| p.path()) {
                waiting += 1;
                if path.is_dir() {
                    let sender = sender.clone();
                    Task::new(path.to_string_lossy().to_string(), sender).run();
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
