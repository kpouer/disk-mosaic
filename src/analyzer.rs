use crate::data::{Data, Kind};
use crate::task::Task;
use crate::ui::data_widget::DataWidget;
use crate::ui::path_bar::PathBar;
use egui::{Context, Widget};
use humansize::DECIMAL;
use log::info;
use std::ops::{Add, AddAssign};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use treemap::{Mappable, TreemapLayout};

#[derive(Debug)]
pub enum Message {
    Data(Data),
    DirectoryScanStart(String),
    DirectoryScanDone(ScanResult),
    Finished,
}

#[derive(Debug, Default, Clone)]
pub struct ScanResult {
    pub(crate) file_count: u64,
    pub(crate) size: u64,
}

impl ScanResult {
    pub(crate) fn add_size(&mut self, size: u64) {
        self.file_count += 1;
        self.size += size;
    }
}

impl Add for ScanResult {
    type Output = ScanResult;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for ScanResult {
    fn add_assign(&mut self, rhs: Self) {
        self.file_count += rhs.file_count;
        self.size += rhs.size;
    }
}

#[derive(Debug)]
pub struct Analyzer {
    modified_in_this_frame: bool,
    data_stack: Vec<Data>,
    rx: Receiver<Message>,
    stopper: Option<Arc<AtomicBool>>,
    handle: Option<thread::JoinHandle<()>>,
    scanning: String,
    scanned_directories: u64,
    scan_result: ScanResult,
}

impl Analyzer {
    pub(crate) fn new(root: PathBuf) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let stopper = Arc::new(AtomicBool::new(false));
        let root_copy = root.clone();
        let stopper_copy = stopper.clone();
        let handle = Some(thread::spawn(move || {
            let start = std::time::Instant::now();
            Task::scan_directory_channel(1, &root_copy, &tx, &stopper_copy);
            tx.send(Message::Finished).unwrap();
            info!("Done in {}s", start.elapsed().as_millis());
        }));
        Self {
            modified_in_this_frame: false,
            data_stack: vec![Data::new_directory(&root, 0)],
            rx,
            stopper: Some(stopper),
            handle,
            scanning: String::new(),
            scanned_directories: 0,
            scan_result: ScanResult::default(),
        }
    }

    pub(crate) fn show(&mut self, ctx: &Context) {
        self.receive_data();

        if let Some(index) = self.show_top_panel(ctx) {
            // index was clicked
            while index < self.data_stack.len() - 1 {
                if let Some(popped_data) = self.data_stack.pop() {
                    if let Some(parent_data) = self.data_stack.get_mut(index) {
                        if let Kind::Dir(children) = &mut parent_data.kind {
                            children.push(popped_data);
                            self.modified_in_this_frame = true;
                        } else {
                            log::error!("Invalid kind ({parent_data:?})");
                        }
                    }
                }
            }
        }

        self.show_central_panel(ctx);
        ctx.request_repaint_after(Duration::from_millis(60));
    }

    fn receive_data(&mut self) {
        for message in self.rx.try_iter() {
            match message {
                Message::DirectoryScanStart(d) => {
                    self.scanning = d;
                    self.scanned_directories += 1;
                }
                Message::DirectoryScanDone(scan_result) => self.scan_result += scan_result,
                Message::Data(data) => {
                    if data.size() > 0.0 {
                        match self.data_stack.last_mut() {
                            Some(current_data) => {
                                current_data.push(data);
                                self.modified_in_this_frame = true;
                            }
                            None => log::error!("Data stack is empty when receiving data"),
                        }
                    }
                }
                Message::Finished => {
                    info!("Scan finished");
                }
            }
        }
    }

    fn show_top_panel(&mut self, ctx: &Context) -> Option<usize> {
        let mut clicked_index = None;
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if let Some(handle) = &self.handle {
                if handle.is_finished() {
                    self.handle = None;
                }
            }
            if self.handle.is_some() {
                ui.horizontal(|ui| {
                    if let Some(stopper) = &self.stopper {
                        if ui.button("Stop").clicked() {
                            stopper.store(true, std::sync::atomic::Ordering::Relaxed);
                        }
                    }
                    ui.label(format!("Scanning: {}", self.scanning));
                });
            } else if let Some(index) = PathBar::new(&self.data_stack).show(ui) {
                clicked_index = Some(index);
            }
            ui.label(format!(
                "Directories: {}, Files: {}, Volume {}",
                self.scanned_directories,
                self.scan_result.file_count,
                humansize::format_size(self.scan_result.size, DECIMAL),
            ));
        });
        clicked_index
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
            let mut clicked_data_index = None;
            if let Some(current_data) = self.data_stack.last_mut() {
                if let Kind::Dir(children) = &mut current_data.kind {
                    if self.modified_in_this_frame {
                        TreemapLayout::new().layout_items(children, rect);
                        self.modified_in_this_frame = false;
                    }
                    children
                        .iter()
                        .enumerate()
                        .filter(|(_, data)| data.bounds.w > 0.0 && data.bounds.h > 0.0)
                        .for_each(|(index, data)| {
                            if DataWidget::new(data).ui(ui).double_clicked()
                                && matches!(data.kind, Kind::Dir(_))
                            {
                                clicked_data_index = Some(index);
                            }
                        });
                }
            }

            if let Some(clicked_index) = clicked_data_index {
                if let Some(current_data) = self.data_stack.last_mut() {
                    if let Kind::Dir(children) = &mut current_data.kind {
                        if clicked_index < children.len() {
                            let taken_data = std::mem::take(&mut children[clicked_index]);
                            self.data_stack.push(taken_data);
                        }
                    }
                }
            }
        });
    }
}
