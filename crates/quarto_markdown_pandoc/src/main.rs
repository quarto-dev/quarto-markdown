/*
 * main.rs
 * Copyright (c) 2025 Posit, PBC
 */

use std::io::{self, Read};
use tree_sitter_qmd::MarkdownParser;
use clap::Parser;

mod traversals;
mod errors;
mod pandoc;
mod filters;
mod writers;
use errors::parse_is_good;

#[derive(Parser)]
#[command(name = "quarto-markdown-pandoc")]
#[command(about = "Convert Quarto markdown to various output formats")]
struct Args {
    #[arg(short = 't', long = "to", default_value = "native")]
    to: String,
}

fn print_whole_tree(cursor: &mut tree_sitter_qmd::MarkdownCursor) {
    let mut depth = 0;
    traversals::topdown_traverse_concrete_tree(cursor, &mut |node, phase| {
        if phase == traversals::TraversePhase::Enter {
            println!("{}{}: {:?}", "  ".repeat(depth), node.kind(), node);
            depth += 1;
        } else {
            depth -= 1;
        }
        true  // continue traversing
    });
}

fn main() {
    let args = Args::parse();
    
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut parser = MarkdownParser::default();
    let input_bytes = input.as_bytes();
    let tree = parser.parse(&input_bytes, None).expect("Failed to parse input");
    let errors = parse_is_good(&tree);
    if !errors.is_empty() {
        let mut cursor = tree.walk();
        // print_whole_tree(&mut cursor, 0);
        for error in errors {
            cursor.goto_id(error);
            eprintln!("{}", errors::error_message(&mut cursor, &input_bytes));
        }
        return;
    }

    print_whole_tree(&mut tree.walk());

    let pandoc = pandoc::treesitter_to_pandoc(&tree, &input_bytes);    
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
