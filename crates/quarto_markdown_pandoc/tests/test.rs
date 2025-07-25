/*
 * test.rs
 * Copyright (c) 2025 Posit, PBC
 */

use tree_sitter_qmd::MarkdownParser;
use quarto_markdown_pandoc::pandoc::{treesitter_to_pandoc};
use quarto_markdown_pandoc::writers;
use std::process::{Command, Stdio};
use std::io::Write;
use glob::glob;

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

fn has_good_pandoc_version() -> bool {
    let output = Command::new("pandoc")
        .arg("--version")
        .output()
        .expect("Failed to execute pandoc command");
    let version_str = String::from_utf8_lossy(&output.stdout);
    version_str.contains("3.6") || version_str.contains("3.7")
}

fn canonicalize_pandoc_ast(ast: &str, from: &str, to: &str) -> String {
    let mut child = Command::new("pandoc")
        .arg("--from").arg(from)
        .arg("--to").arg(to)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start pandoc process");
    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(ast.as_bytes()).expect("Failed to write to stdin");
    let output = child.wait_with_output().expect("Failed to read stdout");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn matches_pandoc_markdown_reader(input: &str) -> bool {
    if !has_good_pandoc_version() {
        return true; // Skip test if pandoc version is not suitable
    }
    let ast = writers::native::write(&treesitter_to_pandoc(&MarkdownParser::default().parse(input.as_bytes(), None).unwrap(), input.as_bytes()));
    let our_ast = canonicalize_pandoc_ast(&ast, "native", "native");
    let pandoc_ast = canonicalize_pandoc_ast(input, "markdown", "native");
    our_ast == pandoc_ast
}

fn matches_pandoc_commonmark_reader(input: &str) -> bool {
    if !has_good_pandoc_version() {
        return true; // Skip test if pandoc version is not suitable
    }
    let ast = writers::native::write(&treesitter_to_pandoc(&MarkdownParser::default().parse(input.as_bytes(), None).unwrap(), input.as_bytes()));
    let our_ast = canonicalize_pandoc_ast(&ast, "native", "native");
    let pandoc_ast = canonicalize_pandoc_ast(input, "commonmark+strikeout+subscript+superscript", "native");
    our_ast == pandoc_ast
}

#[test]
fn unit_test_corpus_matches_pandoc_markdown() {
    assert!(has_good_pandoc_version(), "Pandoc version is not suitable for testing");
    for entry in glob("tests/corpus/markdown/*.qmd").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let input = std::fs::read_to_string(&path).expect("Failed to read file");
                assert!(matches_pandoc_markdown_reader(&input), "File {} does not match pandoc markdown reader", path.display());
            }
            Err(e) => panic!("Error reading glob entry: {}", e),
        }
    }
}

#[test]
fn unit_test_corpus_matches_pandoc_commonmark() {
    assert!(has_good_pandoc_version(), "Pandoc version is not suitable for testing");
    for entry in glob("tests/corpus/commonmark/*.qmd").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let input = std::fs::read_to_string(&path).expect("Failed to read file");
                assert!(matches_pandoc_commonmark_reader(&input), "File {} does not match pandoc commonmark reader", path.display());
            }
            Err(e) => panic!("Error reading glob entry: {}", e),
        }
    }
}