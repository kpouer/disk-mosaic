use egui::{Context, ScrollArea};
use humansize::DECIMAL;
use std::path::PathBuf;
use sysinfo::{Disk, Disks};

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

                if ui.button("Browse for Directory...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        selected_path = Some(path);
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
