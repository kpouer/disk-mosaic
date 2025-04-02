use crate::data::Data;
use eframe::epaint::FontFamily::Proportional;
use eframe::epaint::FontId;
use egui::{Color32, Image, Pos2, Rect, Response, Ui, Vec2, Widget};
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

        if rect.size().x < FONT_SIZE + 2.0 * ui.spacing().item_spacing.x
            || rect.size().y < FONT_SIZE + 2.0 * ui.spacing().item_spacing.y
        {
            return;
        }

        let clip = ui.clip_rect();
        ui.set_clip_rect(rect);

        Image::from(self.data.kind.get_image()).paint_at(
            ui,
            Rect::from_min_size(
                rect.min + ui.spacing().item_spacing,
                Vec2::new(FONT_SIZE, FONT_SIZE),
            ),
        );

        let name = self.data.name();
        if name.is_empty() {
            let galley_name = ui.painter().layout(
                name.into(),
                FONT,
                LABEL_COLOR,
                ui.available_width() - ui.spacing().item_spacing.x * 2.0,
            );
            if galley_name.rect.width() < rect.width() {
                ui.put(
                    Rect::from_min_size(
                        rect.min
                            + Vec2::new(
                                ui.spacing().item_spacing.x * 2.0 + FONT_SIZE,
                                ui.spacing().item_spacing.y,
                            ),
                        galley_name.rect.size(),
                    ),
                    egui::Label::new(galley_name),
                );
                DataSize::new(self.data, rect).ui(ui);
            }
        }
        ui.set_clip_rect(clip);
    }
}

struct DataSize<'a> {
    data: &'a Data,
    rect: Rect,
}

impl<'a> DataSize<'a> {
    fn new(data: &'a Data, rect: Rect) -> Self {
        Self { data, rect }
    }
}

impl Widget for DataSize<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let galley_size = ui.painter().layout(
            humansize::format_size(self.data.size() as u64, DECIMAL),
            FONT,
            LABEL_COLOR,
            ui.available_width(),
        );
        if galley_size.rect.width() < self.rect.width()
            || self.rect.height() < FONT_SIZE * 2.0 + ui.spacing().item_spacing.y * 3.0
        {
            ui.put(
                Rect::from_min_max(
                    Pos2::new(
                        self.rect.min.x + ui.spacing().item_spacing.x,
                        self.rect.max.y - galley_size.rect.height() - ui.spacing().item_spacing.y,
                    ),
                    Pos2::new(self.rect.min.x + galley_size.rect.width(), self.rect.min.y),
                ),
                egui::Label::new(galley_size),
            )
        } else {
            ui.response()
        }
    }
}
