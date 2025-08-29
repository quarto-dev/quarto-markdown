#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
use quarto_markdown_pandoc::readers;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = crate::readers::qmd::read(s.as_bytes(), &mut std::io::sink());
    }
});
