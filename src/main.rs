mod data;
mod disk_analyzer;
mod task;
mod ui;

use disk_analyzer::DiskAnalyzer;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mon application",
        options,
        Box::new(|_| {
            let mut disk_analyzer = DiskAnalyzer::default();
            disk_analyzer.start();
            Ok(Box::new(disk_analyzer))
        }),
    )
}
