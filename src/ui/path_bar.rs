use crate::data::Data;
use egui::{Button, Ui};

#[derive(Debug)]
pub struct PathBar<'a> {
    path_components: &'a [Data],
}

impl<'a> PathBar<'a> {
    pub fn new(path_components: &'a [Data]) -> Self {
        Self { path_components }
    }

    // Return the index of the clicked component
    pub(crate) fn show(&self, ui: &mut Ui) -> Option<usize> {
        let mut clicked_index = None;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            self.path_components
                .iter()
                .enumerate()
                .for_each(|(index, data)| {
                    let is_last = index == self.path_components.len() - 1;
                    if ui
                        .add_enabled(!is_last, Button::new(format!("/{}", data.name.as_str())))
                        .clicked()
                    {
                        clicked_index = Some(index);
                    }
                })
        });
        clicked_index
    }
}
