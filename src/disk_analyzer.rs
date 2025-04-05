use crate::analyzer::Analyzer;
use crate::ui::select_target::SelectTarget;
use log::info;

pub enum AppState {
    SelectDisk(SelectTarget),
    Analyzing(Analyzer),
}

impl Default for AppState {
    fn default() -> Self {
        Self::SelectDisk(SelectTarget::default())
    }
}

#[derive(Default)]
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
            AppState::Analyzing(analyzer) => {
                // show now handles navigation internally
                analyzer.show(ctx);
            }
        }
    }
}
