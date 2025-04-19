mod folder_list_panel;

use crate::settings::{Settings, ThemePreference};
use crate::ui::settings_panel::folder_list_panel::SearchFolderPanel;
use egui::Context;
use std::ops::Index;
use std::sync::{Arc, Mutex};

pub(crate) struct SettingsDialog<'a> {
    settings_context: &'a mut SettingsContext,
    settings: &'a Arc<Mutex<Settings>>,
}

const GEAR: &str = "\u{2699}";

impl<'a> SettingsDialog<'a> {
    pub(crate) fn new(
        settings_context: &'a mut SettingsContext,
        settings: &'a Arc<Mutex<Settings>>,
    ) -> Self {
        Self {
            settings_context,
            settings,
        }
    }

    pub(crate) fn show_button(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        if ui.button(GEAR).clicked() {
            self.settings_context.open = true;
        }
        if self.settings_context.open {
            self.show(ctx);
        }
    }

    fn show(&mut self, ctx: &Context) {
        egui::Window::new("Settings")
            .open(&mut self.settings_context.open)
            .show(ctx, |ui| {
                ui.label("Theme : ");
                let mut settings = self.settings.lock().unwrap();
                ui.horizontal(|ui| {
                    if ui
                        .radio(settings.theme() == ThemePreference::System, "System")
                        .clicked()
                    {
                        settings.set_theme(ThemePreference::System);
                        ctx.set_theme(ThemePreference::System);
                    }
                    if ui
                        .radio(settings.theme() == ThemePreference::Dark, "Dark")
                        .clicked()
                    {
                        settings.set_theme(ThemePreference::Dark);
                        ctx.set_theme(ThemePreference::Dark);
                    }
                    if ui
                        .radio(settings.theme() == ThemePreference::Light, "Light")
                        .clicked()
                    {
                        settings.set_theme(ThemePreference::Light);
                        ctx.set_theme(ThemePreference::Light);
                    }
                });
                ui.separator();
                SearchFolderPanel::new(
                    "ignored_folders",
                    "Ignored Folders",
                    HashListPanel::new(
                        settings.ignored_paths_mut(),
                        &mut self.settings_context.ignored_folders_selection,
                    ),
                )
                .show(ui);
            });
    }
}

struct HashListPanel<'a, T> {
    vec: &'a mut Vec<T>,
    selection: &'a mut Option<usize>,
    dirty: bool,
}

impl<T> HashListPanel<'_, T> {
    pub(crate) fn push(&mut self, item: T) {
        self.vec.push(item);
        self.dirty = true;
    }

    pub(crate) fn remove_selection(&mut self) {
        if let Some(selection) = *self.selection {
            self.vec.remove(selection);
            self.dirty = true;
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.vec.len()
    }
}

impl<T> Index<usize> for HashListPanel<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<'a, T> HashListPanel<'a, T> {
    fn new(vec: &'a mut Vec<T>, selection: &'a mut Option<usize>) -> Self {
        Self {
            vec,
            selection,
            dirty: false,
        }
    }
}

#[derive(Default, Debug)]
pub(crate) struct SettingsContext {
    pub(crate) open: bool,
    pub(crate) ignored_folders_selection: Option<usize>,
}
