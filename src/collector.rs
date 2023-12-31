use swc_ecma_ast::{
    Expr, Ident, JSXElement, JSXElementChild, JSXExpr, JSXExprContainer, JSXFragment, JSXText, Lit,
    MemberExpr, Str,
};
use swc_ecma_visit::Visit;

#[derive(Debug)]
pub struct Collector {
    pub jsx_texts: Vec<JSXText>,
    pub string_literals: Vec<Str>,
    pub variables: Vec<Ident>,
    pub object_properties: Vec<MemberExpr>,
}

impl Visit for Collector {
    fn visit_jsx_element(&mut self, jsx_element: &JSXElement) {
        self.parse_jsx_element(jsx_element);
    }

    fn visit_jsx_fragment(&mut self, jsx_fragment: &JSXFragment) {
        self.parse_jsx_fragment(jsx_fragment);
    }
}

impl Collector {
    pub fn new() -> Self {
        Self {
            jsx_texts: Vec::new(),
            string_literals: Vec::new(),
            variables: Vec::new(),
            object_properties: Vec::new(),
        }
    }

    pub fn trim_content(&mut self) {
        self.jsx_texts = self
            .jsx_texts
            .iter()
            .map(|jsx_text| {
                let mut jsx_text = jsx_text.clone();
                jsx_text.value = jsx_text.value.trim().to_string().into();
                jsx_text
            })
            .collect();
        self.string_literals = self
            .string_literals
            .iter()
            .map(|string_literal| {
                let mut string_literal = string_literal.clone();
                string_literal.value = string_literal.value.trim().to_string().into();
                string_literal
            })
            .collect();
    }

    pub fn parse_jsx_element(&mut self, jsx_element: &JSXElement) {
        for child in &jsx_element.children {
            self.parse_jsx_child(child);
        }
    }

    pub fn parse_jsx_fragment(&mut self, jsx_fragment: &JSXFragment) {
        for child in &jsx_fragment.children {
            self.parse_jsx_child(child);
        }
    }

    fn parse_jsx_child(&mut self, jsx_child: &JSXElementChild) {
        match jsx_child {
            JSXElementChild::JSXText(jsx_text) => {
                if !jsx_text.value.trim().is_empty() {
                    self.jsx_texts.push(jsx_text.clone());
                }
            }
            JSXElementChild::JSXExprContainer(jsx_expr_container) => {
                self.parse_jsx_expression_container(jsx_expr_container)
            }
            JSXElementChild::JSXElement(jsx_element) => self.parse_jsx_element(jsx_element),
            JSXElementChild::JSXFragment(jsx_fragment) => self.parse_jsx_fragment(jsx_fragment),
            JSXElementChild::JSXSpreadChild(_) => {}
        }
    }

    fn parse_jsx_expression_container(&mut self, jsx_expr_container: &JSXExprContainer) {
        match &jsx_expr_container.expr {
            JSXExpr::Expr(expr) => match expr.as_ref() {
                Expr::Lit(lit) => self.parse_literal(lit),
                Expr::Ident(ident) => self.variables.push(ident.clone()),
                Expr::Member(member_expr) => self.object_properties.push(member_expr.clone()),
                _ => {}
            },
            JSXExpr::JSXEmptyExpr(_) => {}
        }
    }

    fn parse_literal(&mut self, lit: &Lit) {
        match lit {
            Lit::Str(string_literal) => {
                if !string_literal.value.trim().is_empty() {
                    self.string_literals.push(string_literal.clone());
                }
            }
            Lit::JSXText(jsx_text) => {
                if !jsx_text.value.trim().is_empty() {
                    self.jsx_texts.push(jsx_text.clone());
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use swc_ecma_ast::{Lit, Str};

    #[test]
    fn parse_literal_with_non_empty_string() {
        let mut collector = Collector::new();
        let lit = Str {
            value: "Hello World".into(),
            span: swc_common::Span::default(),
            raw: None,
        };
        collector.parse_literal(&Lit::Str(lit.clone()));
        assert_eq!(collector.string_literals.len(), 1);
        assert_eq!(collector.string_literals[0], lit);
    }

    #[test]
    fn parse_literal_with_empty_string() {
        let mut collector = Collector::new();
        let lit = Str {
            value: "".into(),
            span: swc_common::Span::default(),
            raw: None,
        };
        collector.parse_literal(&Lit::Str(lit.clone()));
        assert!(collector.string_literals.is_empty());
    }

    #[test]
    fn parse_literal_with_whitespace_string() {
        let mut collector = Collector::new();
        let lit = Str {
            value: " ".into(),
            span: swc_common::Span::default(),
            raw: None,
        };
        collector.parse_literal(&Lit::Str(lit.clone()));
        assert!(collector.string_literals.is_empty());
    }
    #[test]
    fn parse_literal_with_non_empty_jsx_text() {
        let mut collector = Collector::new();
        let jsx_text = JSXText {
            value: "Some JSX Text".into(),
            span: swc_common::Span::default(),
            raw: swc_atoms::Atom::from("Some JSX Text"),
        };
        collector.parse_literal(&Lit::JSXText(jsx_text.clone()));
        assert_eq!(collector.jsx_texts.len(), 1);
        assert_eq!(collector.jsx_texts[0], jsx_text);
    }

    #[test]
    fn parse_literal_with_empty_jsx_text() {
        let mut collector = Collector::new();
        collector.parse_literal(&Lit::JSXText(JSXText {
            value: "".into(),
            span: swc_common::Span::default(),
            raw: swc_atoms::Atom::from(""),
        }));
        assert!(collector.jsx_texts.is_empty());
    }

    #[test]
    fn parse_literal_with_whitespace_jsx_text() {
        let mut collector = Collector::new();
        collector.parse_literal(&Lit::JSXText(JSXText {
            value: "   ".into(),
            span: swc_common::Span::default(),
            raw: swc_atoms::Atom::from("   "),
        }));
        assert!(collector.jsx_texts.is_empty());
    }
}
