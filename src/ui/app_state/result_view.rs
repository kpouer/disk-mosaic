use crate::analysis_result::AnalysisResult;
use crate::data::Kind;
use crate::ui::data_widget::DataWidget;
use crate::ui::path_bar::PathBar;
use egui::{Context, Widget};
use humansize::DECIMAL;
use log::info;
use treemap::TreemapLayout;

#[derive(Debug)]
pub(crate) struct ResultView {
    analysis_result: AnalysisResult,
}

impl ResultView {
    pub fn new(analysis_result: AnalysisResult) -> Self {
        Self { analysis_result }
    }

    pub(crate) fn show(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if let Some(index) = PathBar::new(&self.analysis_result.data_stack).show(ui) {
                while index < self.analysis_result.data_stack.len() - 1 {
                    if let Some(popped_data) = self.analysis_result.data_stack.pop() {
                        if let Some(parent_data) = self.analysis_result.data_stack.last_mut() {
                            if let Kind::Dir(children) = &mut parent_data.kind {
                                info!("Pushing {} into {}", popped_data.name, parent_data.name);
                                children.push(popped_data);
                            } else {
                                log::error!("Invalid kind ({parent_data:?})");
                            }
                        }
                    }
                }
            }
        });
        self.show_central_panel(ctx);
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
            if let Some(current_data) = self.analysis_result.data_stack.last_mut() {
                if let Kind::Dir(children) = &mut current_data.kind {
                    TreemapLayout::new().layout_items(children, rect);
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
                if let Some(current_data) = self.analysis_result.data_stack.last_mut() {
                    if let Kind::Dir(children) = &mut current_data.kind {
                        if clicked_index < children.len() {
                            let taken_data = children.swap_remove(clicked_index); // swapremove because it is faster than a normal remove
                            self.analysis_result.data_stack.push(taken_data);
                        }
                    }
                }
            }
        });
    }
}
