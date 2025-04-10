use egui::{Button, Context, Image, include_image};

pub(crate) struct AboutDialog<'a> {
    open: &'a mut bool,
}

impl<'a> AboutDialog<'a> {
    pub(crate) fn new(open: &'a mut bool) -> Self {
        Self { open }
    }

    pub(crate) fn show_button(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        ui.add_space(ui.available_width() - 18.0);
        if ui
            .add(Button::image(
                Image::from(include_image!("../../assets/question-mark.svg"))
                    .tint(ctx.style().visuals.widgets.noninteractive.text_color()),
            ))
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
