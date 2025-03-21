use crate::data::Data;
use std::path::Path;
use std::sync::mpsc::Sender;

use log::info;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Task<'a> {
    path: PathBuf,
    tx: &'a Sender<Data>,
    stopper: &'a Arc<AtomicBool>,
}

impl<'a> Task<'a> {
    pub fn new(path: PathBuf, tx: &'a Sender<Data>, stopper: &'a Arc<AtomicBool>) -> Self {
        Self { path, tx, stopper }
    }

    pub fn run(self) {
        let Self { path, tx, stopper } = self;
        let mut data = Data::new_directory(path);
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut waiting = Self::scan_directory(&data.path, &sender, stopper);

        while waiting > 0 {
            if let Ok(d) = receiver.recv() {
                waiting -= 1;
                data.children.push(d);
            }
        }
        data.compute_size();
        tx.send(data).unwrap();
    }

    pub fn scan_directory(path: &Path, sender: &Sender<Data>, stopper: &Arc<AtomicBool>) -> usize {
        if let Ok(iter) = path.read_dir() {
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
                    sender.send(Data::new_file(&path)).unwrap();
                }
            });
            ret
        } else {
            println!("Error reading directory: {path:?}");
            0
        }
    }
}
