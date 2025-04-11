use egui::{Button, Context, Image, include_image};

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
        if ui
            .add(Button::image(
                Image::from(include_image!("../../assets/question-mark.svg"))
                    .tint(ctx.style().visuals.widgets.noninteractive.text_color()),
            ))
            .clicked()
        {
            *self.open = true;
        }
        if *self.open {
            self.show(ctx);
        }
    }

    fn show(&mut self, ctx: &Context) {
        egui::Window::new("About Disk Mosaic")
            .open(self.open)
            .show(ctx, |ui| {
                ui.label("Disk Mosaic");
                ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
                ui.label("Created by Matthieu Casanova");
            });
    }
}
