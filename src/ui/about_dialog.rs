use egui::{Button, Context, Image, Pos2, Rect, Vec2, include_image};

pub(crate) struct AboutDialog<'a> {
    open: &'a mut bool,
}

const BUTTON_SIZE: f32 = 18.0;

impl<'a> AboutDialog<'a> {
    pub(crate) fn new(open: &'a mut bool) -> Self {
        Self { open }
    }

    pub(crate) fn show_button(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        ui.add_space(ui.available_width() - BUTTON_SIZE);
        let max_rect = ui.max_rect();
        if ui
            .put(
                Rect::from_min_size(
                    Pos2::new(max_rect.right() - BUTTON_SIZE, max_rect.top()),
                    Vec2::new(BUTTON_SIZE, BUTTON_SIZE),
                ),
                Button::image(
                    Image::from(include_image!("../../assets/question-mark.svg"))
                        .tint(ctx.style().visuals.widgets.noninteractive.text_color()),
                ),
            )
            .clicked()
        {
            *self.open = true;
        }
    }

    pub(crate) fn show(&mut self, ctx: &Context) {
        egui::Window::new("About Disk Analyzer")
            .open(self.open)
            .show(ctx, |ui| {
                ui.label("Disk Analyzer");
                ui.label("Version 0.1.0");
                ui.label("Created by Matthieu Casanova");
            });
    }
}
