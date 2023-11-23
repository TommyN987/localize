use clap::{command, Arg};
use evaluate::Evaluate;
use swc_common::errors::{ColorConfig, Handler};
use swc_ecma_visit::VisitWith;
use walker::walk;

mod analyzer;
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

    let source_files = walk(path);

    let mut parser = parser::Parser::new();

    for source_file in source_files {
        let error_handler = Handler::with_tty_emitter(
            ColorConfig::Always,
            true,
            false,
            Some(parser.source_map.clone()),
        );
        parser
            .parse_file(&source_file)
            .unwrap_or_else(|err| err.into_diagnostic(&error_handler).emit());
    }

    parser.analyzers.iter_mut().for_each(|analyzer| {
        analyzer.module.body.iter().for_each(|child| {
            child.visit_with(&mut analyzer.collector);
        });

        analyzer.collector.trim_content();
    });

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

        for variable in analyzer.collector.variables {
            let report_item = variable.evaluate(&parser.source_map);
            if let Some(report_item) = report_item {
                println!("Filename: {:?}", report_item.filename);
                println!("Line: {:?}", report_item.line);

                report.violations.push(report_item);
                println!("{:?}", variable);
            }
        }

        for prop in analyzer.collector.object_properties {
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
