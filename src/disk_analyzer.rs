use crate::data::Data;
use crate::task::Task;
use crate::ui::data_widget::DataWidget;
use crate::ui::path_bar::PathBar;
use egui::{Context, ProgressBar, Widget};
use log::info;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use treemap::{Mappable, TreemapLayout};

pub struct DiskAnalyzer {
    data: Data,
    root: String,
    rx: Receiver<Data>,
    tx: Sender<Data>,
    stopper: Option<Arc<AtomicBool>>,
    handle: Option<thread::JoinHandle<()>>,
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
            handle: None,
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
        self.handle = Some(thread::spawn(move || {
            let start = std::time::Instant::now();
            Task::scan_directory(&root, &tx, &stopper);
            info!("Done in {}s", start.elapsed().as_millis());
        }));
    }

    fn show_top_panel(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if let Some(handle) = &self.handle {
                if handle.is_finished() {
                    self.handle = None;
                }
            }
            if self.handle.is_some() {
                let progress = ProgressBar::new(0.0).animate(true);
                ui.add(progress);
            } else {
                let parents = self.data.path.ancestors();
                PathBar::new(parents).show(ui);
            }
        });
    }

    fn show_left_panel(&mut self, ctx: &Context) {
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
    }

    fn show_central_panel(&mut self, ctx: &Context) {
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
            self.data
                .children
                .iter_mut()
                .filter(|data| data.bounds.w > 0.0 && data.bounds.h > 0.0)
                .for_each(|data| {
                    if DataWidget::new(data).ui(ui).double_clicked() {
                        clicked_data = Some(data);
                    }
                });
            if let Some(clicked_data) = clicked_data {
                self.data = std::mem::take(clicked_data);
            }
        });
    }
}

impl eframe::App for DiskAnalyzer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut modified = false;
        self.rx
            .try_iter()
            .filter(|data| data.size() > 0.0)
            .for_each(|data| {
                self.data.push(data);
                modified = true;
            });
        if modified {
            self.data.compute_size();
        }
        self.show_left_panel(ctx);
        self.show_top_panel(ctx);

        self.show_central_panel(ctx);
        ctx.request_repaint_after(Duration::from_millis(60))
    }
}
