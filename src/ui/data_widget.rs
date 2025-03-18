use std::path::Path;
use eframe::epaint::FontFamily::Proportional;
use eframe::epaint::FontId;
use egui::{Color32, Pos2, Rect, Ui, Widget};

use crate::data::Data;

pub struct DataWidget<'a> {
    data: &'a Data,
}

impl<'a> DataWidget<'a> {
    pub fn new(data: &'a Data) -> Self {
        Self { data }
    }
}

const HOVER_ZOOMING: f32 = 10.0;

impl<'a> Widget for DataWidget<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let mut rect = Rect::from_min_max(
            egui::emath::Pos2::new(self.data.bounds.x as f32, self.data.bounds.y as f32),
            egui::emath::Pos2::new(
                (self.data.bounds.x + self.data.bounds.w) as f32,
                (self.data.bounds.y + self.data.bounds.h) as f32,
            ),
        );
        let response = ui.allocate_rect(rect, egui::Sense::click());
        let zoomed = response.hovered() || response.clicked();

        if zoomed {
            rect.min.x -= HOVER_ZOOMING;
            rect.min.y -= HOVER_ZOOMING;
            rect.max.x += HOVER_ZOOMING;
            rect.max.y += HOVER_ZOOMING;
        }
        ui.painter().rect(
            rect,
            egui::epaint::CornerRadius::ZERO,
            self.data.color,
            egui::Stroke::default(),
            egui::StrokeKind::Inside,
        );
        self.draw_label(ui, rect, zoomed);

        response
    }
}

impl<'a> DataWidget<'a> {
    fn draw_label(self, ui: &mut Ui, mut rect: Rect, zoomed: bool) {
        let font_id = FontId::new(18.0, Proportional);
        if zoomed {
            rect.min.x += HOVER_ZOOMING;
            rect.min.y += HOVER_ZOOMING;
            rect.max.x -= HOVER_ZOOMING;
            rect.max.y -= HOVER_ZOOMING;
        }
        let path = Path::new(&self.data.name);

        let layout = ui.painter().layout(
            path.file_name().unwrap().to_string_lossy().to_string(),
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
    }
}
