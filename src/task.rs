use crate::data::Data;
use std::path::Path;
use std::sync::mpsc::Sender;

use log::info;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Task {
    path: PathBuf,
    tx: Sender<Data>,
    stopper: Arc<AtomicBool>,
}

impl Task {
    pub fn new(path: PathBuf, tx: Sender<Data>, stopper: Arc<AtomicBool>) -> Self {
        Self { path, tx, stopper }
    }

    pub fn run(&self) {
        let mut data = Data::new_directory(self.path.clone());
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut waiting = Self::scan_directory(&self.path, &sender, &self.stopper);

        while waiting > 0 {
            if let Ok(d) = receiver.recv() {
                waiting -= 1;
                data.children.push(d);
            }
        }
        data.compute_size();
        self.tx.send(data).unwrap();
    }

    pub fn scan_directory(path: &Path, sender: &Sender<Data>, stopper: &Arc<AtomicBool>) -> usize {
        let mut waiting = 0;
        if let Ok(iter) = path.read_dir() {
            for path in iter.flatten().map(|p| p.path()) {
                if stopper.load(Ordering::Relaxed) {
                    info!("Stop requested");
                    break;
                }
                waiting += 1;
                if path.is_dir() {
                    let sender = sender.clone();
                    Task::new(path, sender, stopper.clone()).run();
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
