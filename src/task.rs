use crate::data::Data;
use std::io::ErrorKind;
use std::path::Path;
use std::sync::mpsc::Sender;

use crate::analyzer::Message;
use log::{debug, info};
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Task<'a> {
    path: PathBuf,
    tx: &'a Sender<Message>,
    stopper: &'a Arc<AtomicBool>,
}

impl<'a> Task<'a> {
    pub fn new(path: PathBuf, tx: &'a Sender<Message>, stopper: &'a Arc<AtomicBool>) -> Self {
        Self { path, tx, stopper }
    }

    pub fn run(self) {
        let Self { path, tx, stopper } = self;
        // Pass the Task's path to the constructor
        let mut data = Data::new_directory(&path);
        let (sender, receiver) = std::sync::mpsc::channel();
        // Pass the Task's path to scan_directory
        let mut waiting = Self::scan_directory(&path, &sender, stopper);

        while waiting > 0 {
            if let Ok(message) = receiver.recv() {
                match message {
                    Message::Data(d) => {
                        waiting -= 1;
                        data.children.push(d);
                    }
                    Message::Finished => {
                        debug!("Finished");
                    }
                }
            }
        }
        data.compute_size();
        tx.send(Message::Data(data)).unwrap();
    }

    pub fn scan_directory(
        path: &Path,
        sender: &Sender<Message>,
        stopper: &Arc<AtomicBool>,
    ) -> usize {
        match path.read_dir() {
            Ok(iter) => {
                let vec = iter.collect::<Vec<_>>();
                let ret = vec.len();
                vec.par_iter().flatten().map(|p| p.path()).for_each(|path| {
                    if stopper.load(Ordering::Relaxed) {
                        info!("Stop requested");
                        return;
                    }
                    if path.is_dir() {
                        Task::new(path, sender, stopper).run();
                    } else {
                        sender.send(Message::Data(Data::new_file(&path))).unwrap();
                    }
                });
                ret
            }
            Err(e) => {
                match e.kind() {
                    ErrorKind::PermissionDenied => {}
                    _ => debug!("Error reading directory: {path:?}, {e:?}"),
                }
                0
            }
        }
    }
}
