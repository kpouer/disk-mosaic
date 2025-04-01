use crate::data::Data;
use crate::task::Task;
use crate::ui::data_widget::DataWidget;
use crate::ui::path_bar::PathBar;
use egui::{Context, ProgressBar, Widget};
use log::info;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use treemap::{Mappable, TreemapLayout};

pub struct Analyzer {
    data: Data,
    rx: Receiver<Data>,
    stopper: Option<Arc<AtomicBool>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Analyzer {
    pub(crate) fn new(root: PathBuf) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let stopper = Arc::new(AtomicBool::new(false));
        let root_copy = root.clone();
        let stopper_copy = stopper.clone();
        let handle = Some(thread::spawn(move || {
            let start = std::time::Instant::now();
            Task::scan_directory(&root_copy, &tx, &stopper_copy);
            info!("Done in {}s", start.elapsed().as_millis());
        }));
        Self {
            data: Data::new_directory(root.clone()),
            rx,
            stopper: Some(stopper),
            handle,
        }
    }

    pub(crate) fn show(&mut self, ctx: &Context) {
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
        self.show_top_panel(ctx);

        self.show_central_panel(ctx);
        ctx.request_repaint_after(Duration::from_millis(60))
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
