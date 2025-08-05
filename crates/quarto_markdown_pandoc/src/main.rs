#![allow(dead_code)]

/*
 * main.rs
 * Copyright (c) 2025 Posit, PBC
 */

use clap::Parser;
use std::io::{self, Read, Write};

mod errors;
mod filters;
mod pandoc;
mod readers;
mod traversals;
mod utils;
mod writers;
use utils::output::VerboseOutput;

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

    let result = readers::qmd::read(input.as_bytes(), &mut output_stream);
    let pandoc = match result {
        Ok(p) => p,
        Err(error_messages) => {
            for msg in error_messages {
                eprintln!("{}", msg);
            }
            std::process::exit(1);
        }
    };

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
