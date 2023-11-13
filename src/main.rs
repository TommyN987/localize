use clap::{command, Arg};
use swc_common::errors::{ColorConfig, Handler};
use swc_ecma_visit::VisitWith;
use walker::walk;

mod jsx_analyzer;
mod parser;
mod walker;

fn main() {
    let matches = command!()
        .arg(Arg::new("path").short('p').long("path").required(true))
        .get_matches();

    let path: &String = matches.get_one("path").unwrap();

    let tsx_vec = walk(path);

    let mut parser = parser::Parser::new();

    for tsx in tsx_vec {
        let error_handler = Handler::with_tty_emitter(
            ColorConfig::Always,
            true,
            false,
            Some(parser.source_map.clone()),
        );
        let module = parser.parse_file(&tsx);
        match module {
            Ok(module) => {
                let mut analyzer = jsx_analyzer::JSXAnalyzer::new(&tsx.to_str().unwrap());
                for child in module.body {
                    child.visit_with(&mut analyzer);
                }
                parser.analyzers.push(analyzer);
            }
            Err(error) => {
                error.into_diagnostic(&error_handler).emit();
            }
        }
    }
    for analyzer in parser.analyzers {
        println!("{:?}", analyzer);
    }
}
