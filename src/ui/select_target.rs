use egui::{Context, ScrollArea};
use home::home_dir;
use humansize::DECIMAL;
use std::path::PathBuf;
use sysinfo::{Disk, Disks};

#[derive(Debug)]
pub(crate) struct SelectTarget {
    disks: Vec<DiskLabel>,
}

impl Default for SelectTarget {
    fn default() -> Self {
        let disks = Disks::new_with_refreshed_list()
            .iter()
            .map(DiskLabel::from)
            .collect();
        Self { disks }
    }
}

#[derive(Debug)]
struct DiskLabel {
    mount_point: PathBuf,
    label: String,
}

impl From<&Disk> for DiskLabel {
    fn from(disk: &Disk) -> Self {
        let mount_point = disk.mount_point();
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total.saturating_sub(available);

        let label = format!(
            "{} ({}) - Used: {} / Total: {} (Available: {})",
            disk.mount_point().display(),
            disk.name().to_string_lossy(),
            humansize::format_size(used, DECIMAL),
            humansize::format_size(total, DECIMAL),
            humansize::format_size(available, DECIMAL)
        );

        Self {
            mount_point: mount_point.to_path_buf(),
            label,
        }
    }
}

impl SelectTarget {
    pub fn show(&self, ctx: &Context) -> Option<PathBuf> {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                let mut selected_path = None;
                ui.heading("Select Scan Target");
                ui.separator();

                ui.label("Available Disks/Mounts:");

                ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    if self.disks.is_empty() {
                        ui.label("(No disks found by sysinfo)");
                    } else {
                        self.disks.iter().for_each(|disk| {
                            if ui.button(&disk.label).clicked() {
                                selected_path = Some(disk.mount_point.to_owned());
                            }
                        });
                    }
                });

                ui.separator();

                if ui.button("Home Folder").clicked() {
                    if let Some(path) = home_dir() {
                        selected_path = Some(path);
                    } else {
                        // Optional: Log or display an error if home dir not found
                        log::error!("Could not determine home directory.");
                    }
                }

                if let Some(path) = &selected_path {
                    ui.separator();
                    ui.label(format!("Selected: {}", path.display()));
                }

                selected_path
            })
            .inner
    }
}
