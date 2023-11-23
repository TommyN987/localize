#[derive(Debug, Default)]
pub struct Report {
    pub violations: Vec<ReportItem>,
    pub suspicions: Vec<ReportItem>,
}

#[derive(Debug, Default)]
pub struct ReportItem {
    pub filename: String,
    pub line: usize,
    pub string: String,
}
