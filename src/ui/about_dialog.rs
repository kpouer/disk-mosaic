use egui::Context;

pub(crate) struct AboutDialog<'a> {
    open: &'a mut bool,
}

impl<'a> AboutDialog<'a> {
    pub(crate) fn new(open: &'a mut bool) -> Self {
        Self { open }
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
