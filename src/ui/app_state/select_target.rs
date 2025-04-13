use crate::service::storage_manager::StorageManager;
use crate::service::storage_manager::storage::Storage;
use crate::ui::about_dialog::AboutDialog;
use egui::{
    Button, Context, CursorIcon, Frame, Label, ProgressBar, Rect, Response, RichText, ScrollArea,
    Sense, Ui, UiBuilder, Vec2, Widget,
};
use home::home_dir;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub(crate) struct SelectTarget {
    storage_manager: StorageManager,
    about_open: bool,
}

impl SelectTarget {
    pub fn show(&mut self, ctx: &Context) -> Option<PathBuf> {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                let mut selected_path = None;
                ui.horizontal(|ui| {
                    ui.heading("Select Scan Target");
                    AboutDialog::new(&mut self.about_open).show_button(ctx, ui);
                });
                ui.separator();

                ui.label("Available Disks/Mounts:");

                ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    if self.storage_manager.is_empty() {
                        ui.label("(No disks found by sysinfo)");
                    } else {
                        self.storage_manager.iter().for_each(|disk| {
                            if StorageWidget::new(disk).ui(ui).clicked() {
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

                if ui.button("Select Folder...").clicked() {
                    selected_path = rfd::FileDialog::new().pick_folder();
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

struct StorageWidget<'a> {
    storage: &'a Storage,
}

impl<'a> StorageWidget<'a> {
    fn new(storage: &'a Storage) -> Self {
        Self { storage }
    }
}

const HEIGHT: f32 = 48.0;
const PROGRESS_WIDTH: f32 = 100.0;

impl Widget for StorageWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let response = ui
            .scope_builder(
                UiBuilder::new()
                    .id_salt(&self.storage.mount_point)
                    .sense(Sense::click()),
                |ui| {
                    let response = ui.response();
                    let visuals = ui.style().interact(&response);
                    let text_color = visuals.text_color();

                    Frame::canvas(ui.style())
                        .fill(visuals.bg_fill.gamma_multiply(0.3))
                        .stroke(visuals.fg_stroke)
                        .inner_margin(ui.spacing().menu_margin)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::Image::new(self.storage.icon())
                                        .fit_to_exact_size(Vec2::new(HEIGHT, HEIGHT)),
                                );
                                ui.label(&self.storage.name)
                                    .on_hover_cursor(CursorIcon::Default);
                                // ui.add_space(ui.available_width() - PROGRESS_WIDTH);
                                // ui.add(
                                //     ProgressBar::new(self.storage.progress())
                                //         .desired_width(PROGRESS_WIDTH)
                                //         .desired_height(36.0)
                                //         .show_percentage(),
                                // );
                            });
                        });
                },
            )
            .response;
        if response.hovered() {
            egui::show_tooltip(ui.ctx(), ui.layer_id(), egui::Id::new("my_tooltip"), |ui| {
                ui.label("Helpful text");
            });
        }
        response
    }
}
