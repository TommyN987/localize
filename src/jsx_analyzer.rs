use swc_ecma_ast::{
    Expr, JSXAttr, JSXAttrOrSpread, JSXElement, JSXElementChild, JSXExpr, JSXExprContainer,
    JSXFragment, JSXOpeningElement, JSXText, Lit, Str,
};

#[derive(Debug)]
pub struct JSXAnalyzer {
    pub jsx_texts: Vec<JSXText>,
    pub string_literals: Vec<Str>,
    pub props: Vec<JSXAttr>,
}

impl JSXAnalyzer {
    pub fn new() -> Self {
        Self {
            jsx_texts: Vec::new(),
            string_literals: Vec::new(),
            props: Vec::new(),
        }
    }

    pub fn analyze_jsx_element(&mut self, jsx_element: &JSXElement) {
        self.analyze_jsx_opening_element(&jsx_element.opening);
        for child in &jsx_element.children {
            self.analyze_jsx_child(child);
        }
    }

    pub fn analyze_jsx_fragment(&mut self, jsx_fragment: &JSXFragment) {
        for child in &jsx_fragment.children {
            self.analyze_jsx_child(child);
        }
    }

    fn analyze_jsx_child(&mut self, jsx_child: &JSXElementChild) {
        match jsx_child {
            JSXElementChild::JSXText(jsx_text) => {
                self.jsx_texts.push(jsx_text.clone());
            }
            JSXElementChild::JSXExprContainer(jsx_expr_container) => {
                self.analyze_jsx_expression_container(jsx_expr_container)
            }
            JSXElementChild::JSXElement(jsx_element) => self.analyze_jsx_element(jsx_element),
            JSXElementChild::JSXFragment(jsx_fragment) => self.analyze_jsx_fragment(jsx_fragment),
            JSXElementChild::JSXSpreadChild(_) => {}
        }
    }

    fn analyze_jsx_opening_element(&mut self, jsx_opening_element: &JSXOpeningElement) {
        for attr in &jsx_opening_element.attrs {
            match attr {
                JSXAttrOrSpread::JSXAttr(attr) => {
                    if let Some(_value) = &attr.value {
                        self.props.push(attr.clone())
                    }
                }
                JSXAttrOrSpread::SpreadElement(_) => {}
            }
        }
    }

    fn analyze_jsx_expression_container(&mut self, jsx_expr_container: &JSXExprContainer) {
        match &jsx_expr_container.expr {
            JSXExpr::Expr(expr) => match expr.as_ref() {
                Expr::Lit(lit) => self.analyze_literal(lit),
                _ => {}
            },
            JSXExpr::JSXEmptyExpr(_) => {}
        }
    }

    fn analyze_literal(&mut self, lit: &Lit) {
        match lit {
            Lit::Str(string_literal) => {
                self.string_literals.push(string_literal.clone());
            }
            Lit::JSXText(jsx_text) => {
                self.jsx_texts.push(jsx_text.clone());
            }
            _ => {}
        }
    }
}
