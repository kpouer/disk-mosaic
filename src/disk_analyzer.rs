use crate::data::Data;
use rand::Rng;
use treemap::TreemapLayout;
use egui::Widget;
use crate::ui::data_widget::DataWidget;

struct TreemapDataWidget<'a> {
    data: &'a [Data],
}

pub struct DiskAnalyzer {
    data: Vec<Data>,
}

impl Default for DiskAnalyzer {
    fn default() -> Self {
        let mut rnd = rand::rng();
        let mut data = vec![];
        for i in 0..100 {
            data.push(Data::new(format!("Item {}", i), rnd.random::<f64>()));
        }
        Self { data }
    }
}

impl eframe::App for DiskAnalyzer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Left Panel");
            ui.label("This is the left panel content.");
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
