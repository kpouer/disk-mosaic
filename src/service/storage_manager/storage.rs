use egui::{ImageSource, include_image};
use std::path::PathBuf;
use sysinfo::{Disk, DiskKind};

#[derive(Debug)]
pub struct Storage {
    pub(crate) mount_point: PathBuf,
    pub(crate) name: String,
    pub(crate) available: u64,
    pub(crate) total: u64,
    pub(crate) removable: bool,
    kind: DiskKind,
}

impl From<&Disk> for Storage {
    fn from(disk: &Disk) -> Self {
        Self {
            mount_point: disk.mount_point().to_path_buf(),
            name: disk.name().to_string_lossy().to_string(),
            available: disk.available_space(),
            total: disk.total_space(),
            removable: disk.is_removable(),
            kind: disk.kind(),
        }
    }
}

impl Storage {
    pub(crate) fn progress(&self) -> f32 {
        let total = self.total;
        let used = total - self.available;
        used as f32 / total as f32
    }

    pub(crate) fn icon(&self) -> ImageSource {
        match self.kind {
            DiskKind::HDD => include_image!("../../../assets/hdd.svg"),
            DiskKind::SSD => include_image!("../../../assets/ssd.svg"),
            DiskKind::Unknown(_) => {
                if self.removable {
                    include_image!("../../../assets/removable.svg")
                } else {
                    include_image!("../../../assets/ssd.svg")
                }
            }
        }
    }
}
