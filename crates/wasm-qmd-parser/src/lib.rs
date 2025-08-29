// For `vsnprintf()` and `fprintf()`, which are variadic.
// Otherwise rustc yells at us that we need to enable this.
#![feature(c_variadic)]

// Provide rust implementation of blessed stdlib functions to
// tree-sitter itself and any grammars that have `scanner.c`.
// Here is the list blessed for `scanner.c` usage:
// https://github.com/tree-sitter/tree-sitter/blob/master/lib/src/wasm/stdlib-symbols.txt
// But note that we need a few extra for tree-sitter itself.
#[cfg(target_arch = "wasm32")]
pub mod c_shim;

mod utils;

use std::io;

use quarto_markdown_pandoc::readers::qmd;
use quarto_markdown_pandoc::utils::output::VerboseOutput;
use quarto_markdown_pandoc::writers::json;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-qmd-parser!");
}

#[wasm_bindgen]
pub fn parse_qmd(input: &str) -> String {
    let mut output = VerboseOutput::Sink(io::sink());
    let result = qmd::read(input.as_bytes(), &mut output).unwrap();
    let mut buf = Vec::new();
    let _ = json::write(&result, &mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}
