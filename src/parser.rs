use std::path::PathBuf;

use swc_common::{comments::SingleThreadedComments, sync::Lrc, SourceMap};
use swc_ecma_ast::{EsVersion, Module};
use swc_ecma_parser::{error::Error, lexer::Lexer, Parser as SWCParser, Syntax, TsConfig};

use crate::collector::Collector;

pub struct Parser {
    pub source_map: Lrc<SourceMap>,
    pub analyzers: Vec<Collector>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            source_map: Lrc::new(SourceMap::default()),
            analyzers: Vec::new(),
        }
    }

    pub fn parse_file(&self, filename: &PathBuf) -> Result<Module, Error> {
        let fm = self
            .source_map
            .load_file(filename)
            .expect("Failed to load file");
        let comments = SingleThreadedComments::default();
        let lexer = Lexer::new(
            Syntax::Typescript(TsConfig {
                tsx: true,
                dts: false,
                decorators: true,
                ..Default::default()
            }),
            EsVersion::latest(),
            (&*fm).into(),
            Some(&comments),
        );
        let mut parser = SWCParser::new_from(lexer);
        let module = parser.parse_typescript_module();
        module
    }
}
