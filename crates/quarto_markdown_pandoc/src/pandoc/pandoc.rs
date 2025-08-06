/*
 * pandoc.rs
 * Copyright (c) 2025 Posit, PBC
 */

pub use crate::pandoc::attr::Attr;
pub use crate::pandoc::block::{Block, Blocks};
pub use crate::pandoc::caption::Caption;
pub use crate::pandoc::inline::{Citation, Inline, Inlines};
pub use crate::pandoc::list::ListAttributes;

/*
 * A data structure that mimics Pandoc's `data Pandoc` type.
 * This is used to represent the parsed structure of a Quarto Markdown document.
 */

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pandoc {
    // eventually, meta
    pub blocks: Blocks,
}
