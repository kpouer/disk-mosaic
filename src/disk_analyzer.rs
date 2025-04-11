use crate::ui::app_state::analyzer::{Analyzer, AnalyzerUpdate}; // Added AnalyzerUpdate
use crate::ui::app_state::result_view::ResultView;
use crate::ui::app_state::select_target::SelectTarget;
use log::info;

#[derive(Debug)]
enum AppState {
    SelectDisk(SelectTarget),
    Analyzing(Analyzer),
    Analyzed(ResultView),
}

impl Default for AppState {
    fn default() -> Self {
        Self::SelectDisk(SelectTarget::default())
    }
}

#[derive(Debug, Default)]
pub struct DiskAnalyzerApp {
    state: AppState,
}

impl eframe::App for DiskAnalyzerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match &mut self.state {
            AppState::SelectDisk(select_target) => {
                if let Some(selected_path) = select_target.show(ctx) {
                    info!("Selected path: {selected_path:?}");
                    self.state = AppState::Analyzing(Analyzer::new(selected_path));
                }
            }
            AppState::Analyzing(analyzer) => match analyzer.show(ctx) {
                AnalyzerUpdate::Finished => {
                    info!("Analysis finished, transitioning to ResultView");
                    let analysis_result = std::mem::take(&mut analyzer.analysis_result);
                    self.state = AppState::Analyzed(ResultView::new(analysis_result));
                }
                AnalyzerUpdate::GoBack => {
                    info!("Back requested from Analyzer, transitioning to SelectTarget");
                    self.state = AppState::SelectDisk(SelectTarget::default());
                }
                AnalyzerUpdate::Running => {}
            },
            AppState::Analyzed(result_view) => {
                if result_view.show(ctx) {
                    info!("Back requested from ResultView, transitioning to SelectTarget");
                    self.state = AppState::SelectDisk(SelectTarget::default());
                }
            }
        }
    }
}
