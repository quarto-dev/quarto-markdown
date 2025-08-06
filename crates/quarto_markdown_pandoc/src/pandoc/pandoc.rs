/*
 * pandoc.rs
 * Copyright (c) 2025 Posit, PBC
 */

pub use crate::pandoc::block::Blocks;

/*
 * A data structure that mimics Pandoc's `data Pandoc` type.
 * This is used to represent the parsed structure of a Quarto Markdown document.
 */

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pandoc {
    // eventually, meta
    pub blocks: Blocks,
}
