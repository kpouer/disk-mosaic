mod data;
mod disk_analyzer;

use disk_analyzer::DiskAnalyzer;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mon application",
        options,
        Box::new(|_| Ok(Box::<DiskAnalyzer>::default())),
    )
}
