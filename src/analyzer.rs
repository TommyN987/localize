use swc_ecma_ast::Module;

use crate::collector::Collector;

pub struct Analyzer {
    pub module: Module,
    pub collector: Collector,
}

impl Analyzer {
    pub fn new(module: Module) -> Self {
        Self {
            module,
            collector: Collector::new(),
        }
    }
}
