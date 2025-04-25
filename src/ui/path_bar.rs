use crate::data::Data;
use egui::{Button, Ui, Vec2};

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
            {
                let spacing = ui.spacing_mut();
                spacing.item_spacing.x = 0.0;
                spacing.button_padding = Vec2::ZERO;
            }
            self.path_components[1..]
                .iter()
                .enumerate()
                .for_each(|(index, data)| {
                    let is_last = index == self.path_components.len() - 1;
                    if ui
                        .add_enabled(!is_last, Button::new(format!("/{}", &data.name)))
                        .clicked()
                    {
                        clicked_index = Some(index);
                    }
                })
        });
        clicked_index
    }
}
