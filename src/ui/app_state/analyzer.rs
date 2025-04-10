use crate::analysis_result::AnalysisResult;
use crate::data::Data;
use crate::task::Task;
use crate::ui::about_dialog::AboutDialog;
use crate::ui::path_bar::PathBar;
use crate::ui::treemap_panel::TreeMapPanel;
use egui::{Context, Label};
use humansize::DECIMAL;
use log::info;
use std::ops::{Add, AddAssign};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use treemap::Mappable;

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
    pub(crate) analysis_result: AnalysisResult,
    rx: Receiver<Message>,
    stopper: Option<Arc<AtomicBool>>,
    handle: Option<thread::JoinHandle<()>>,
    scanning: String,
    scanned_directories: u64,
    scan_result: ScanResult,
    about_open: bool,
}

impl Analyzer {
    /// Create a new analyzer.
    /// The analyzer will scan the given directory and all subdirectories in a thread.
    pub(crate) fn new(root: PathBuf) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let stopper = Arc::new(AtomicBool::new(false));
        let root_copy = root.clone();
        let stopper_copy = stopper.clone();
        let handle = Some(thread::spawn(move || {
            let start = std::time::Instant::now();
            Task::scan_directory_channel(&root_copy, &tx, &stopper_copy);
            tx.send(Message::Finished).unwrap();
            info!("Done in {}ms", start.elapsed().as_millis());
        }));
        Self {
            analysis_result: AnalysisResult::new(vec![Data::new_directory(&root)]),
            rx,
            stopper: Some(stopper),
            handle,
            scanning: String::new(),
            scanned_directories: 0,
            scan_result: ScanResult::default(),
            about_open: false,
        }
    }

    pub(crate) fn show(&mut self, ctx: &Context) -> bool {
        let finished = self.receive_data();

        if let Some(index) = self.show_top_panel(ctx) {
            self.analysis_result.selected_index(index);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            TreeMapPanel::new(&mut self.analysis_result).show(ui);
        });
        ctx.request_repaint_after(Duration::from_millis(60));
        finished
    }

    fn receive_data(&mut self) -> bool {
        let mut finished = false;
        for message in self.rx.try_iter() {
            match message {
                Message::DirectoryScanStart(d) => {
                    self.scanning = d;
                    self.scanned_directories += 1;
                }
                Message::DirectoryScanDone(scan_result) => self.scan_result += scan_result,
                Message::Data(data) => {
                    if data.size() > 0.0 {
                        match self.analysis_result.data_stack.last_mut() {
                            Some(current_data) => current_data.push(data),
                            None => log::error!("Data stack is empty when receiving data"),
                        }
                    }
                }
                Message::Finished => {
                    info!("Scan finished");
                    finished = true;
                }
            }
        }
        finished
    }

    fn show_top_panel(&mut self, ctx: &Context) -> Option<usize> {
        let mut clicked_index = None;
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(handle) = &self.handle {
                    if handle.is_finished() {
                        self.handle = None;
                    }
                }
                if self.handle.is_some() {
                    if let Some(stopper) = &self.stopper {
                        if ui.button("Stop").clicked() {
                            stopper.store(true, std::sync::atomic::Ordering::Relaxed);
                        }
                    }
                    ui.label(format!("Scanning: {}", self.scanning));
                } else {
                    if let Some(index) = PathBar::new(&self.analysis_result.data_stack).show(ui) {
                        clicked_index = Some(index);
                    }
                    let scanning_label = Label::new(format!(
                        "Directories: {}, Files: {}, Volume {}",
                        self.scanned_directories,
                        self.scan_result.file_count,
                        humansize::format_size(self.scan_result.size, DECIMAL)
                    ));
                    ui.add(scanning_label);
                }
                AboutDialog::new(&mut self.about_open).show_button(ctx, ui);
            });
        });

        AboutDialog::new(&mut self.about_open).show(ctx);
        clicked_index
    }
}
