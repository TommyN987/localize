use crate::report::ReportItem;
use swc_common::{sync::Lrc, SourceMap};
use swc_ecma_ast::{Expr, JSXAttr, JSXAttrName, JSXAttrValue, JSXText, Lit, Str};

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

impl Evaluate for Str {
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
        let prop_name = match &self.name {
            JSXAttrName::Ident(ident) => Some(ident.sym.to_string()),
            JSXAttrName::JSXNamespacedName(namespaced_name) => {
                Some(namespaced_name.name.sym.to_string())
            }
        };

        match &self.value {
            Some(value) => match &value {
                JSXAttrValue::Lit(lit) => match &lit {
                    Lit::Str(str) => {
                        // println!("{:?}: {:?}", prop_name.unwrap(), str.value);
                        None
                    }
                    _ => None,
                },
                JSXAttrValue::JSXExprContainer(jsx_expr_container) => {
                    match &jsx_expr_container.expr {
                        swc_ecma_ast::JSXExpr::JSXEmptyExpr(_) => todo!(),
                        swc_ecma_ast::JSXExpr::Expr(expr) => match expr.as_ref() {
                            Expr::Ident(ident) => {
                                println!("{:?}: {:?}", prop_name.unwrap(), ident.sym);
                                None
                            }
                            _ => None,
                        },
                    }
                }
                _ => None,
            },
            None => None,
        }
    }
}
