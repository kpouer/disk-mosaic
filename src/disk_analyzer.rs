use crate::data::Data;
use crate::task::Task;
use std::thread;
use egui::Widget;
use treemap::TreemapLayout;
use crate::ui::data_widget::DataWidget;

pub struct DiskAnalyzer {
    data: Vec<Data>,
    root: String,
}

impl Default for DiskAnalyzer {
    fn default() -> Self {
        let root = match home::home_dir() {
            None => "/".to_string(),
            Some(home) => home.as_path().to_string_lossy().to_string(),
        };
        let root = "/Users/kpouer/dev/rust".to_string();
        Self { data: Vec::new(), root }
    }
}

impl DiskAnalyzer {
    pub(crate) fn start(&mut self) {
        let task = Task::new(self.root.clone());
        let future = thread::spawn(move || task.run());
        self.data = future.join().unwrap().children;
    }
}

impl eframe::App for DiskAnalyzer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Root");
                ui.text_edit_singleline(&mut self.root);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let clip_rect = ui.clip_rect();
            let rect = treemap::Rect::from_points(
                clip_rect.left() as f64,
                clip_rect.top() as f64,
                clip_rect.width() as f64,
                clip_rect.height() as f64,
            );
            TreemapLayout::new().layout_items(&mut self.data, rect);

            self.data.iter().for_each(|data| {
                DataWidget::new(data).ui(ui);
            });
        });
    }
}
