use crate::data::Data;
use eframe::epaint::FontFamily::Proportional;
use eframe::epaint::FontId;
use egui::{Color32, Pos2, Rect, Ui, Widget};
use humansize::DECIMAL;
use treemap::Mappable;

pub struct DataWidget<'a> {
    data: &'a Data,
}

impl<'a> DataWidget<'a> {
    pub fn new(data: &'a Data) -> Self {
        Self { data }
    }
}

const HOVER_ZOOMING: f32 = 10.0;
const FONT_SIZE: f32 = 18.0;
const LABEL_COLOR: Color32 = Color32::WHITE;
const FONT: FontId = FontId::new(FONT_SIZE, Proportional);

impl Widget for DataWidget<'_> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let mut rect = Rect::from_min_max(
            Pos2::new(self.data.bounds.x as f32, self.data.bounds.y as f32),
            Pos2::new(
                (self.data.bounds.x + self.data.bounds.w) as f32,
                (self.data.bounds.y + self.data.bounds.h) as f32,
            ),
        );
        let response = ui.allocate_rect(rect, egui::Sense::click());
        let zoomed = response.hovered() || response.clicked();

        if zoomed {
            rect = rect.expand(HOVER_ZOOMING);
        }
        ui.painter().rect(
            rect,
            egui::epaint::CornerRadius::ZERO,
            self.data.color,
            egui::Stroke::new(1.0, Color32::BLACK),
            egui::StrokeKind::Inside,
        );
        self.draw_label(ui, rect, zoomed);

        response
    }
}

impl DataWidget<'_> {
    fn draw_label(&self, ui: &mut Ui, mut rect: Rect, zoomed: bool) {
        if zoomed {
            rect = rect.shrink(HOVER_ZOOMING);
        }

        let galley_name = ui.painter().layout(
            self.data.file_name().into(),
            FONT,
            LABEL_COLOR,
            ui.available_width() - ui.spacing().item_spacing.x * 2.0,
        );
        if galley_name.rect.width() < rect.width() {
            ui.put(
                Rect::from_min_size(
                    rect.min + ui.spacing().item_spacing,
                    galley_name.rect.size(),
                ),
                egui::Label::new(galley_name),
            );
            let galley_size = ui.painter().layout(
                humansize::format_size(self.data.size() as u64, DECIMAL),
                FONT,
                LABEL_COLOR,
                ui.available_width(),
            );
            if galley_size.rect.width() < rect.width() {
                ui.put(
                    Rect::from_min_max(
                        Pos2::new(
                            rect.min.x + ui.spacing().item_spacing.x,
                            rect.max.y - galley_size.rect.height() - ui.spacing().item_spacing.y,
                        ),
                        Pos2::new(rect.min.x + galley_size.rect.width(), rect.min.y),
                    ),
                    egui::Label::new(galley_size),
                );
            }
        }
    }
}
