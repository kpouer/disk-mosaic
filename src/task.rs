use crate::analyzer::Message;
use crate::data::{Data, Kind};
use log::{debug, info, warn};
use rayon::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;

pub struct Task<'a> {
    /// the depth that will be set for the children of that task
    depth: u16,
    path: PathBuf,
    tx: &'a Sender<Message>,
    stopper: &'a Arc<AtomicBool>,
}

impl<'a> Task<'a> {
    pub fn new(
        depth: u16,
        path: PathBuf,
        tx: &'a Sender<Message>,
        stopper: &'a Arc<AtomicBool>,
    ) -> Self {
        Self {
            depth,
            path,
            tx,
            stopper,
        }
    }

    pub fn run(self) {
        let Self {
            depth,
            path,
            tx,
            stopper,
        } = self;

        let mut data = Data::new_directory(&path, self.depth);

        match Self::scan_directory_recursive(&path, self.depth, stopper) {
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

    // New recursive scanning function returning results directly
    fn scan_directory_recursive(
        path: &Path,
        depth: u16,
        stopper: &Arc<AtomicBool>,
    ) -> Result<Vec<Data>, Error> {
        let entries = match path.read_dir() {
            Ok(iter) => iter.collect::<Result<Vec<_>, Error>>()?,
            Err(e) => {
                if e.kind() != ErrorKind::PermissionDenied {
                    debug!("Error reading directory: {path:?}, {e:?}");
                }
                return Ok(Vec::new());
            }
        };

        let children: Vec<Data> = entries
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
                let small_file_data = Data {
                    depth,
                    name: "Remaining".to_string(),
                    kind: Kind::File,
                    size: metadata.len(),
                    color: Data::next_color(),
                    ..Default::default()
                };
                if metadata.is_dir() {
                    match Self::scan_directory_recursive(&entry_path, depth + 1, stopper) {
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
                    Some(Data::new_file(&entry_path, depth + 1))
                } else {
                    // Ignore symlinks, sockets, etc.
                    None
                }
            })
            .collect();

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
                        Task::new(depth, path, sender, stopper).run();
                    } else {
                        sender
                            .send(Message::Data(Data::new_file(&path, depth)))
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
