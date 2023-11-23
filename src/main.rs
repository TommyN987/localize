use clap::{command, Arg};
use evaluate::Evaluate;
use swc_common::errors::{ColorConfig, Handler};
use swc_ecma_visit::VisitWith;
use walker::walk;

mod collector;
mod evaluate;
mod parser;
mod report;
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
                let mut analyzer = collector::Collector::new();
                for child in module.body {
                    child.visit_with(&mut analyzer);
                }
                analyzer.trim_content();
                parser.analyzers.push(analyzer);
            }
            Err(error) => {
                error.into_diagnostic(&error_handler).emit();
            }
        }
    }
    let mut report = report::Report::default();
    for analyzer in parser.analyzers {
        // for jsx_test in analyzer.jsx_texts {
        //     let report_item = jsx_test.evaluate(&parser.source_map);
        //     if let Some(report_item) = report_item {
        //         report.violations.push(report_item);
        //     }
        // }

        // for string_literal in analyzer.string_literals {
        //     let report_item = string_literal.evaluate(&parser.source_map);
        //     if let Some(report_item) = report_item {
        //         report.violations.push(report_item);
        //     }
        // }

        for variable in analyzer.variables {
            let report_item = variable.evaluate(&parser.source_map);
            if let Some(report_item) = report_item {
                println!("Filename: {:?}", report_item.filename);
                println!("Line: {:?}", report_item.line);

                report.violations.push(report_item);
                println!("{:?}", variable);
            }
        }

        for prop in analyzer.object_properties {
            let report_item = prop.evaluate(&parser.source_map);
            if let Some(report_item) = report_item {
                println!("Filename: {:?}", report_item.filename);
                println!("Line: {:?}", report_item.line);
                report.violations.push(report_item);
                println!("{:?}", prop);
            }
        }
    }
}
