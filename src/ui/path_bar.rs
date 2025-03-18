use egui::Ui;
use std::path::{Ancestors, Path};

pub struct PathBar<'a> {
    parents: Ancestors<'a>,
}

impl<'a> PathBar<'a> {
    pub fn new(parents: Ancestors<'a>) -> Self {
        Self { parents }
    }

    pub(crate) fn show(&self, ui: &mut Ui) -> Option<&Path> {
        let mut ret = None;
        ui.horizontal(|ui| {
            for parent in self.parents {
                if let Some(parent_name) = parent.file_name() {
                    if ui
                        .button(format!("/{}", parent.to_string_lossy().to_string()))
                        .clicked()
                    {
                        ret = Some(parent);
                    }
                }
            }
        });
        ret
    }
}
