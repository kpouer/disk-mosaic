use crate::service::storage_manager::StorageManager;
use crate::service::storage_manager::storage::Storage;
use crate::settings::Settings;
use crate::ui::about_dialog::AboutDialog;
use crate::ui::settings_panel::SettingsDialog;
use egui::{Button, Context, Response, ScrollArea, Ui, Vec2, Widget};
use home::home_dir;
use humansize::DECIMAL;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub(crate) struct SelectTarget {
    settings: Arc<Mutex<Settings>>,
    storage_manager: StorageManager,
    about_open: bool,
    settings_open: bool,
}

impl SelectTarget {
    pub(crate) fn new(settings: Arc<Mutex<Settings>>) -> Self {
        Self {
            settings,
            storage_manager: Default::default(),
            about_open: false,
            settings_open: false,
        }
    }

    pub fn show(&mut self, ctx: &Context) -> Option<PathBuf> {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Select Scan Target");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    AboutDialog::new(&mut self.about_open).show_button(ctx, ui);
                    SettingsDialog::new(&mut self.settings_open, &self.settings)
                        .show_button(ctx, ui);
                });
            });
        });
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                let mut selected_path = None;
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

                if ui
                    .add_sized(
                        Vec2::new(ui.available_width(), HEIGHT),
                        Button::new("Home Folder"),
                    )
                    .clicked()
                {
                    if let Some(path) = home_dir() {
                        selected_path = Some(path);
                    } else {
                        log::error!("Could not determine home directory.");
                    }
                }

                if ui
                    .add_sized(
                        Vec2::new(ui.available_width(), HEIGHT),
                        Button::new("Select Folder..."),
                    )
                    .clicked()
                {
                    selected_path = rfd::FileDialog::new().pick_folder();
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

impl Widget for StorageWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let image = egui::Image::new(self.storage.icon())
            .tint(ui.style().visuals.widgets.noninteractive.text_color())
            .fit_to_exact_size(Vec2::new(HEIGHT, HEIGHT));
        let button = Button::image_and_text(image, self.storage.name());
        let response = ui.add_sized(Vec2::new(ui.available_width(), HEIGHT), button);
        if response.hovered() {
            egui::show_tooltip(ui.ctx(), ui.layer_id(), egui::Id::new("my_tooltip"), |ui| {
                ui.heading(self.storage.name());
                ui.separator();
                ui.label(format!(
                    "Mount: {}",
                    self.storage.mount_point.to_string_lossy()
                ));
                ui.label(format!(
                    "{} / {}",
                    humansize::format_size(self.storage.total - self.storage.available, DECIMAL),
                    humansize::format_size(self.storage.total, DECIMAL)
                ));
                if self.storage.removable {
                    ui.label("Removable");
                }
            });
        }
        response
    }
}
