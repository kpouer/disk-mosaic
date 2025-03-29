mod data;
mod disk_analyzer;
mod task;
mod ui;

use disk_analyzer::DiskAnalyzer;
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
            let mut disk_analyzer = DiskAnalyzer::default();
            disk_analyzer.start();
            Ok(Box::new(disk_analyzer))
        }),
    )
}
