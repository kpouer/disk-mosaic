mod analyzer;
mod data;
mod disk_analyzer;
mod task;
mod ui;

use disk_analyzer::DiskAnalyzerApp;
use egui_extras::install_image_loaders;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Disk Analyzer",
        options,
        Box::new(|ctx| {
            install_image_loaders(&ctx.egui_ctx);
            egui_solarized::install(&ctx.egui_ctx);
            Ok(Box::new(DiskAnalyzerApp::default()))
        }),
    )
}
