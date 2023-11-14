use crate::report::ReportItem;
use swc_common::{sync::Lrc, SourceMap};
use swc_ecma_ast::{JSXAttr, JSXText, Str};

pub trait Evaluate {
    fn evaluate(&self, sourcemap: &Lrc<SourceMap>) -> Option<ReportItem>;
}

impl Evaluate for JSXText {
    fn evaluate(&self, sourcemap: &Lrc<SourceMap>) -> Option<ReportItem> {
        let sf_and_line = sourcemap.lookup_line(self.span.lo());
        match sf_and_line {
            Ok(sf_and_line) => {
                let report_item = ReportItem {
                    filename: sf_and_line.sf.name.to_string(),
                    line: sf_and_line.line,
                    string: self.value.to_string(),
                    prop_name: None,
                };
                Some(report_item)
            }
            Err(_) => None,
        }
    }
}

impl Evaluate for JSXAttr {
    fn evaluate(&self, sourcemap: &Lrc<SourceMap>) -> Option<ReportItem> {
        println!("JSXAttr: {:?}", self.name);
        None
    }
}

impl Evaluate for Str {
    fn evaluate(&self, sourcemap: &Lrc<SourceMap>) -> Option<ReportItem> {
        println!("Str: {:?}", self.value);
        None
    }
}
