/*
 * test.rs
 * Copyright (c) 2025 Posit, PBC
 */

use tree_sitter_qmd::MarkdownParser;
use quarto_markdown_pandoc::pandoc::{treesitter_to_pandoc};
use quarto_markdown_pandoc::writers;

#[test]
fn unit_test_simple_qmd_parses() {
    let inputs = [
        "_hello_",
        "**bold**",
        "$e=mc^2$",
        "$$e=mc^2$$",
        ];
    for input in inputs {
        let mut parser = MarkdownParser::default();
        let input_bytes = input.as_bytes();
        let tree = parser.parse(input_bytes, None).expect("Failed to parse input");
        println!("{}", writers::native::write(&treesitter_to_pandoc(&tree, &input_bytes)));
        assert!(true, "Parsed successfully");
    }
}
