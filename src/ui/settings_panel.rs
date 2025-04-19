use crate::settings::{Settings, ThemePreference};
use egui::Context;
use std::sync::{Arc, Mutex};

pub(crate) struct SettingsDialog<'a> {
    open: &'a mut bool,
    settings: &'a Arc<Mutex<Settings>>,
}

const GEAR: &str = "\u{2699}";

impl<'a> SettingsDialog<'a> {
    pub(crate) fn new(open: &'a mut bool, settings: &'a Arc<Mutex<Settings>>) -> Self {
        Self { open, settings }
    }

    pub(crate) fn show_button(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        if ui.button(GEAR).clicked() {
            *self.open = true;
        }
        if *self.open {
            self.show(ctx);
        }
    }

    fn show(&mut self, ctx: &Context) {
        egui::Window::new("Settings")
            .open(self.open)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Theme : ");
                    {
                        let mut settings = self.settings.lock().unwrap();
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
                    }
                });
            });
    }
}
