use eframe::epaint::FontFamily::Proportional;
use eframe::epaint::FontId;
use egui::{Color32, Pos2, Rect, RichText, Ui, Widget};

use crate::data::Data;

pub struct DataWidget<'a> {
    data: &'a Data,
}

impl<'a> DataWidget<'a> {
    pub fn new(data: &'a Data) -> Self {
        Self { data }
    }
}

impl<'a> Widget for DataWidget<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let rect = Rect::from_min_max(
            egui::emath::Pos2::new(self.data.bounds.x as f32, self.data.bounds.y as f32),
            egui::emath::Pos2::new(
                (self.data.bounds.x + self.data.bounds.w) as f32,
                (self.data.bounds.y + self.data.bounds.h) as f32,
            ),
        );

        ui.painter().rect(
            rect,
            egui::epaint::CornerRadius::ZERO,
            self.data.color,
            egui::Stroke::default(),
            egui::StrokeKind::Inside,
        );
        let font_id = FontId::new(12.5, Proportional);
        let layout = ui.painter().layout(
            self.data.name.clone(),
            font_id,
            Color32::BLACK,
            ui.available_width(),
        );
        if layout.rect.width() < rect.width() {
            ui.put(
                Rect::from_min_max(
                    rect.min,
                    Pos2::new(
                        rect.min.x + layout.rect.width(),
                        rect.min.y + layout.rect.height(),
                    ),
                ),
                egui::Label::new(layout),
            );
        }

        ui.allocate_rect(rect, egui::Sense::hover())
    }
}
