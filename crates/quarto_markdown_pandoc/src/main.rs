/*
 * main.rs
 * Copyright (c) 2025 Posit, PBC
 */

use clap::Parser;
use std::io::{self, Read, Write};
use tree_sitter_qmd::MarkdownParser;

mod errors;
mod filters;
mod pandoc;
mod traversals;
mod utils;
mod writers;
use errors::parse_is_good;

#[derive(Parser, Debug)]
#[command(name = "quarto-markdown-pandoc")]
#[command(about = "Convert Quarto markdown to various output formats")]
struct Args {
    #[arg(short = 't', long = "to", default_value = "native")]
    to: String,

    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

fn print_whole_tree<T: Write>(cursor: &mut tree_sitter_qmd::MarkdownCursor, buf: &mut T) {
    let mut depth = 0;
    traversals::topdown_traverse_concrete_tree(cursor, &mut |node, phase| {
        if phase == traversals::TraversePhase::Enter {
            writeln!(buf, "{}{}: {:?}", "  ".repeat(depth), node.kind(), node).unwrap();
            depth += 1;
        } else {
            depth -= 1;
        }
        true // continue traversing
    });
}

enum VerboseOutput {
    Stderr(io::Stderr),
    Sink(io::Sink),
}

impl Write for VerboseOutput {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            VerboseOutput::Stderr(stderr) => stderr.write(buf),
            VerboseOutput::Sink(sink) => sink.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            VerboseOutput::Stderr(stderr) => stderr.flush(),
            VerboseOutput::Sink(sink) => sink.flush(),
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut input = String::new();
    let mut output_stream = if args.verbose {
        VerboseOutput::Stderr(io::stderr())
    } else {
        VerboseOutput::Sink(io::sink())
    };
    io::stdin().read_to_string(&mut input).unwrap();
    if !input.ends_with("\n") {
        eprintln!("(Warning) Adding missing newline to end of input.");
        //
        input.push('\n'); // ensure the input ends with a newline
    }

    let mut parser = MarkdownParser::default();
    let input_bytes = input.as_bytes();
    let tree = parser
        .parse(&input_bytes, None)
        .expect("Failed to parse input");
    let errors = parse_is_good(&tree);
    if !errors.is_empty() {
        let mut cursor = tree.walk();
        for error in errors {
            cursor.goto_id(error);
            eprintln!("{}", errors::error_message(&mut cursor, &input_bytes));
        }
        return;
    }

    print_whole_tree(&mut tree.walk(), &mut output_stream);
    let pandoc = pandoc::treesitter_to_pandoc(&mut output_stream, &tree, &input_bytes);
    let output = match args.to.as_str() {
        "json" => writers::json::write(&pandoc),
        "native" => writers::native::write(&pandoc),
        _ => {
            eprintln!("Unknown output format: {}", args.to);
            return;
        }
    };
    println!("{}", output);
}
