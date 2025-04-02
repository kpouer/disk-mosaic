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

pub enum Message {
    Data(Data),
    Finished,
}

pub struct Analyzer {
    data: Data,
    current_path: PathBuf,
    rx: Receiver<Message>,
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
            tx.send(Message::Finished).unwrap();
            info!("Done in {}s", start.elapsed().as_millis());
        }));
        Self {
            data: Data::new_directory(&root),
            current_path: root,
            rx,
            stopper: Some(stopper),
            handle,
        }
    }

    pub(crate) fn show(&mut self, ctx: &Context) {
        let mut modified = false;
        for message in self.rx.try_iter() {
            match message {
                Message::Data(data) => {
                    if data.size() > 0.0 {
                        self.data.push(data);
                        modified = true;
                    }
                }
                Message::Finished => {
                    info!("Scan finished");
                }
            }
        }

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
                let parents = self.current_path.ancestors();
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
                        // Store a reference to the clicked data to avoid borrowing issues
                        clicked_data = Some(data as *mut Data); // Use raw pointer temporarily
                    }
                });

            // Process click after iteration
            if let Some(clicked_data_ptr) = clicked_data {
                // Safety: We know the pointer is valid because it came from the iterator
                // and we haven't modified the Vec structure since.
                // We need to take ownership, so we use mem::take on the original data.
                // This requires finding the element again or using the pointer carefully.
                // A safer approach might be to store the index or name and find it again.
                // Let's try finding by name (assuming names are unique within a directory).

                // Get the name from the clicked data (unsafe block needed for dereference)
                let clicked_name = unsafe { (*clicked_data_ptr).name() };

                // Find the index of the clicked item
                if let Some(index) = self
                    .data
                    .children
                    .iter()
                    .position(|d| d.name() == clicked_name)
                {
                    // Update the current path *before* taking the data
                    self.current_path.push(clicked_name);
                    // Take ownership of the clicked data
                    let taken_data = std::mem::take(&mut self.data.children[index]);
                    // Replace the analyzer's root data
                    self.data = taken_data;
                    // Note: The original Vec now contains a default Data instance at 'index'.
                    // This might be okay if we always navigate deeper, but could be an issue
                    // if we implement 'up' navigation later without rebuilding the parent.
                    // For now, this matches the previous logic's effect.
                }
            }
        });
    }
}
