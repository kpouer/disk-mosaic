use crate::data::Data;
use eframe::epaint::FontFamily::Proportional;
use eframe::epaint::FontId;
use egui::{Color32, Image, Pos2, Rect, Response, Ui, Vec2, Widget};
use humansize::DECIMAL;
use treemap::Mappable;

#[derive(Debug)]
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
        if zoomed {
            rect = rect.shrink(HOVER_ZOOMING);
        }
        DataLabel::new(self.data, rect).ui(ui);
        response
    }
}

#[derive(Debug)]
struct DataLabel<'a> {
    data: &'a Data,
    rect: Rect,
}

impl<'a> DataLabel<'a> {
    fn new(data: &'a Data, rect: Rect) -> Self {
        Self { data, rect }
    }
}

impl Widget for DataLabel<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self { data, rect } = self;
        if rect.size().x < FONT_SIZE + 2.0 * ui.spacing().item_spacing.x
            || rect.size().y < FONT_SIZE + 2.0 * ui.spacing().item_spacing.y
        {
            return ui.response();
        }

        let clip = ui.clip_rect();
        ui.set_clip_rect(rect);

        Image::from(data.kind.get_image()).paint_at(
            ui,
            Rect::from_min_size(
                rect.min + ui.spacing().item_spacing,
                Vec2::new(FONT_SIZE, FONT_SIZE),
            ),
        );

        let name = data.name();
        if !name.is_empty() {
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
                DataSize::new(data, rect).ui(ui);
            }
        }
        ui.set_clip_rect(clip);
        ui.response()
    }
}

#[derive(Debug)]
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
        let Self { data, rect } = self;
        let galley_size = ui.painter().layout(
            humansize::format_size(data.size() as u64, DECIMAL),
            FONT,
            LABEL_COLOR,
            ui.available_width(),
        );
        if galley_size.rect.width() < rect.width()
            || rect.height() < FONT_SIZE * 2.0 + ui.spacing().item_spacing.y * 3.0
        {
            ui.put(
                Rect::from_min_max(
                    Pos2::new(
                        rect.min.x + ui.spacing().item_spacing.x,
                        rect.max.y - galley_size.rect.height() - ui.spacing().item_spacing.y,
                    ),
                    Pos2::new(rect.min.x + galley_size.rect.width(), rect.min.y),
                ),
                egui::Label::new(galley_size),
            )
        } else {
            ui.response()
        }
    }
}
