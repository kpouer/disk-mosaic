use crate::data::Data;
use crate::task::Task;
use crate::ui::data_widget::DataWidget;
use crate::ui::path_bar::PathBar;
use egui::Widget;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use treemap::TreemapLayout;

pub struct DiskAnalyzer {
    data: Data,
    root: String,
    rx: Receiver<Data>,
    tx: Sender<Data>,
    stopper: Option<Arc<AtomicBool>>,
}

impl Default for DiskAnalyzer {
    fn default() -> Self {
        let root = match home::home_dir() {
            None => "/".to_string(),
            Some(home) => home.as_path().to_string_lossy().to_string(),
        };
        let root = "/Users/kpouer/dev/rust".to_string();
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            data: Data::new_directory(PathBuf::from(&root)),
            root,
            rx,
            tx,
            stopper: None,
        }
    }
}

impl DiskAnalyzer {
    pub fn start(&mut self) {
        let root = PathBuf::from(&self.root);
        self.data = Data::new_directory(root.clone());
        let tx = self.tx.clone();
        let stopper = Arc::new(AtomicBool::new(false));
        self.stopper = Some(stopper.clone());
        thread::spawn(move || Task::scan_directory(&root, &tx, &stopper));
    }
}

impl eframe::App for DiskAnalyzer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut modified = false;
        while let Ok(data) = self.rx.try_recv() {
            self.data.push(data);
            modified = true;
        }
        if modified {
            self.data.compute_size();
        }
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Root");
                if ui.text_edit_singleline(&mut self.root).changed() {
                    if let Some(stopper) = &self.stopper {
                        stopper.store(true, Ordering::Relaxed);
                        self.start();
                    }
                }
            });
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let parents = self.data.path.ancestors();
            PathBar::new(parents).show(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let clip_rect = ui.clip_rect();
            let rect = treemap::Rect::from_points(
                clip_rect.left() as f64,
                clip_rect.top() as f64,
                clip_rect.width() as f64,
                clip_rect.height() as f64,
            );
            TreemapLayout::new().layout_items(&mut self.data.children, rect);
            let mut clicked_data = None;
            self.data.children.iter_mut().for_each(|data| {
                if DataWidget::new(data).ui(ui).double_clicked() {
                    clicked_data = Some(data);
                }
            });
            if let Some(clicked_data) = clicked_data {
                self.data = std::mem::take(clicked_data);
            }
        });
        ctx.request_repaint_after(Duration::from_millis(60))
    }
}
