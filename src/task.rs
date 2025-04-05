use crate::analyzer::Message;
use crate::data::{Data, Kind};
use log::{debug, info, warn};
use rayon::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub struct Task<'a> {
    /// the depth that will be set for the children of that task
    depth: u16,
    path: PathBuf,
    tx: &'a Sender<Message>,
    stopper: &'a Arc<AtomicBool>,
    sender: Sender<Message>,
}

const BIG_FILE_THRESHOLD: u64 = 10000000;

impl<'a> Task<'a> {
    pub fn new(
        depth: u16,
        path: PathBuf,
        tx: &'a Sender<Message>,
        stopper: &'a Arc<AtomicBool>,
        sender: Sender<Message>,
    ) -> Self {
        Self {
            depth,
            path,
            tx,
            stopper,
            sender,
        }
    }

    pub fn run(self) {
        let Self {
            depth,
            path,
            tx,
            stopper,
            sender,
        } = self;

        let mut data = Data::new_directory(&path, self.depth);

        match Self::scan_directory_recursive(&path, self.depth, stopper, &sender) {
            Ok(children) => {
                data.set_nodes(children);
            }
            Err(e) => {
                warn!("Error scanning directory {:?}: {}", path, e);
            }
        }

        if let Err(e) = tx.send(Message::Data(data)) {
            warn!("Failed to send data message: {}", e);
        }
    }

    fn scan_directory_recursive(
        path: &Path,
        depth: u16,
        stopper: &Arc<AtomicBool>,
        sender: &Sender<Message>,
    ) -> Result<Vec<Data>, Error> {
        sender
            .send(Message::Progression(path.to_string_lossy().to_string()))
            .unwrap();
        let entries = match path.read_dir() {
            Ok(iter) => {
                let iter = iter.flatten();
                #[cfg(target_os = "macos")]
                let iter = iter.filter(|p| !p.path().starts_with("/System/Volumes"));
                iter.collect::<Vec<_>>()
            }
            Err(e) => {
                if e.kind() != ErrorKind::PermissionDenied {
                    debug!("Error reading directory: {path:?}, {e:?}");
                }
                return Ok(Vec::new());
            }
        };

        let small_file_data = Arc::new(Mutex::new(Data {
            depth,
            name: "Remaining".to_string(),
            kind: Kind::SmallFiles,
            size: 0,
            color: Data::next_color(),
            ..Default::default()
        }));
        let mut children: Vec<Data> = entries
            .par_iter()
            .filter_map(|entry| {
                if stopper.load(Ordering::Relaxed) {
                    info!("Stop requested during recursive scan");
                    return None;
                }

                let entry_path = entry.path();
                let metadata = match entry.metadata() {
                    Ok(m) => m,
                    Err(e) => {
                        debug!("Failed to get metadata for {:?}: {}", entry_path, e);
                        return None;
                    }
                };
                if metadata.is_dir() {
                    match Self::scan_directory_recursive(&entry_path, depth + 1, stopper, sender) {
                        Ok(grandchildren) => {
                            let mut dir_data = Data::new_directory(&entry_path, depth + 1);
                            dir_data.set_nodes(grandchildren);
                            Some(dir_data)
                        }
                        Err(e) => {
                            warn!(
                                "Error recursively scanning directory {:?}: {}",
                                entry_path, e
                            );
                            None
                        }
                    }
                } else if metadata.is_file() {
                    let size = Data::get_file_size(&entry_path);
                    if size < BIG_FILE_THRESHOLD {
                        let mut d = small_file_data.lock().unwrap();
                        d.size += size;
                        None
                    } else {
                        Some(Data::new_file(&entry_path, size, depth + 1))
                    }
                } else {
                    // Ignore symlinks, sockets, etc.
                    None
                }
            })
            .collect();

        let small_file_data = small_file_data.lock().unwrap();
        if small_file_data.size > 0 {
            children.push(small_file_data.clone());
        }
        Ok(children)
    }

    pub fn scan_directory_channel(
        depth: u16,
        path: &Path,
        sender: &Sender<Message>,
        stopper: &Arc<AtomicBool>,
    ) {
        match path.read_dir() {
            Ok(iter) => {
                let vec = iter.collect::<Vec<_>>();
                vec.iter().flatten().map(|p| p.path()).for_each(|path| {
                    if stopper.load(Ordering::Relaxed) {
                        info!("Stop requested");
                        return;
                    }
                    if path.is_dir() {
                        sender
                            .send(Message::Progression(path.to_string_lossy().to_string()))
                            .unwrap();
                        Task::new(depth, path, sender, stopper, sender.clone()).run();
                    } else {
                        let size = Data::get_file_size(&path);
                        sender
                            .send(Message::Data(Data::new_file(&path, size, depth)))
                            .unwrap();
                    }
                });
            }
            Err(e) => match e.kind() {
                ErrorKind::PermissionDenied => {}
                _ => debug!("Error reading directory: {path:?}, {e:?}"),
            },
        }
    }
}
