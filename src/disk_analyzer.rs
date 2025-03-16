use crate::data::Data;
use eframe::epaint::{CornerRadius, Stroke, StrokeKind};
use egui::emath;
use rand::Rng;
use treemap::TreemapLayout;
use egui::Widget;

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
        for i in 0..10 {
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
            let layout = TreemapLayout::new();
            let available_size = ui.available_size();
            let rect = treemap::Rect::from_points(
                0.0,
                0.0,
                available_size.x as f64,
                available_size.y as f64,
            );
            layout.layout_items(&mut self.data, rect);

            self.data.iter().for_each(|data| {
                let rect = egui::emath::Rect::from_min_max(
                    emath::Pos2::new(data.bounds.x as f32, data.bounds.y as f32),
                    emath::Pos2::new(
                        (data.bounds.x + data.bounds.w) as f32,
                        (data.bounds.y + data.bounds.h) as f32,
                    ),
                );
                ui.painter().rect(
                    rect,
                    CornerRadius::ZERO,
                    data.color,
                    Stroke::default(),
                    StrokeKind::Inside,
                );
            });
        });
    }
}
