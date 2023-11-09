use clap::{command, Arg};
use std::fs;
use walker::walk;

mod analyzer;
mod parser;
mod walker;

fn main() {
    let matches = command!()
        .arg(Arg::new("path").short('p').long("path").required(true))
        .get_matches();

    let path: &String = matches.get_one("path").unwrap();

    let tsx_vec = walk(path);

    for tsx in tsx_vec {
        let content = fs::read_to_string(tsx).expect("Something went wrong reading the file");
        let module = parser::parse_tsx_file(&content);

        // let mut analyzer = analyzer::Analyzer::new();
        // analyzer.analyze_module(&module);

        // println!("Strings: {:?}", analyzer.result.strings);
        // println!("Template strings: {:?}", analyzer.result.template_strings);
        // println!("JSX texts: {:?}", analyzer.result.jsx_texts);
    }
}
