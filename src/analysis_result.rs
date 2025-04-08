use crate::data::Data;

#[derive(Debug)]
pub(crate) struct AnalysisResult {
    pub(crate) data_stack: Vec<Data>,
}

impl AnalysisResult {
    pub(crate) fn new(data_stack: Vec<Data>) -> AnalysisResult {
        Self { data_stack }
    }
}
