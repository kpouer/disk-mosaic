use crate::analysis_result::AnalysisResult;
use crate::ui::path_bar::PathBar;
use crate::ui::treemap_panel::TreeMapPanel;
use egui::Context;

#[derive(Debug)]
pub(crate) struct ResultView {
    analysis_result: AnalysisResult,
}

impl ResultView {
    pub fn new(analysis_result: AnalysisResult) -> Self {
        Self { analysis_result }
    }

    pub(crate) fn show(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if let Some(index) = PathBar::new(&self.analysis_result.data_stack).show(ui) {
                self.analysis_result.selected_index(index);
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            TreeMapPanel::new(&mut self.analysis_result).show(ui);
        });
    }
}
