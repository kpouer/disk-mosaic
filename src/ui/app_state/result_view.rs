use crate::analysis_result::AnalysisResult;
use crate::ui::about_dialog::AboutDialog;
use crate::ui::path_bar::PathBar;
use crate::ui::treemap_panel::TreeMapPanel;
use egui::Context;

#[derive(Debug)]
pub(crate) struct ResultView {
    analysis_result: AnalysisResult,
    about_open: bool,
}

impl ResultView {
    pub fn new(analysis_result: AnalysisResult) -> Self {
        Self {
            analysis_result,
            about_open: false,
        }
    }
}

impl ResultView {
    pub(crate) fn show(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(index) = PathBar::new(&self.analysis_result.data_stack).show(ui) {
                    self.analysis_result.selected_index(index);
                }
                if ui.button("About").clicked() {
                    self.about_open = true;
                }
            });
        });
        AboutDialog::new(&mut self.about_open).show(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            TreeMapPanel::new(&mut self.analysis_result).show(ui);
        });
    }
}
