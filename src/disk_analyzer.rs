use crate::analyzer::Analyzer;
use log::info;

#[derive(Default)]
pub enum AppState {
    #[default]
    SelectDisk,
    Analyzing(Analyzer),
}

#[derive(Default)]
pub struct DiskAnalyzerApp {
    state: AppState,
}

impl eframe::App for DiskAnalyzerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match &mut self.state {
            AppState::SelectDisk => {
                if let Some(selected_path) = crate::ui::select_target::show(ctx) {
                    info!("Selected path: {selected_path:?}");
                    self.state = AppState::Analyzing(Analyzer::new(selected_path));
                }
            }
            AppState::Analyzing(analyzer) => analyzer.show(ctx),
        }
    }
}
