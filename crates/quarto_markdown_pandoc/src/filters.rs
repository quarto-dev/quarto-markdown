/*
 * filters.rs
 * Copyright (c) 2025 Posit, PBC
 */

use crate::pandoc;

// filters are destructive and take ownership of the input

type InlineFilterFn<T> = Option<fn(T) -> (Vec<pandoc::Inline>, bool)>;

#[derive(Default)]
pub struct Filter {
    pub inline: InlineFilterFn<pandoc::Inline>,

    pub str: InlineFilterFn<pandoc::Str>,
    pub emph: InlineFilterFn<pandoc::Emph>,
    pub underline: InlineFilterFn<pandoc::Underline>,
    pub strong: InlineFilterFn<pandoc::Strong>,
    pub strikeout: InlineFilterFn<pandoc::Strikeout>,
    pub superscript: InlineFilterFn<pandoc::Superscript>,
    pub subscript: InlineFilterFn<pandoc::Subscript>,
    pub small_caps: InlineFilterFn<pandoc::SmallCaps>,
    pub quoted: InlineFilterFn<pandoc::Quoted>,
    pub cite: InlineFilterFn<pandoc::Cite>,
    pub code: InlineFilterFn<pandoc::Code>,
    pub space: InlineFilterFn<pandoc::Space>,
    pub soft_break: InlineFilterFn<pandoc::SoftBreak>,
    pub line_break: InlineFilterFn<pandoc::LineBreak>,
    pub math: InlineFilterFn<pandoc::Math>,
    pub raw_inline: InlineFilterFn<pandoc::RawInline>,
    pub link: InlineFilterFn<pandoc::Link>,
    pub image: InlineFilterFn<pandoc::Image>,
    pub note: InlineFilterFn<pandoc::Note>,
    pub span: InlineFilterFn<pandoc::Span>,
    pub shortcode: InlineFilterFn<pandoc::Shortcode>,

    pub block: Option<fn(pandoc::Block) -> pandoc::Block>,
}

// pub fn topdown_traverse(doc: pandoc::Pandoc, filter: &Filter) -> pandoc::Pandoc {
//     let mut result = pandoc::Pandoc::default();
//     for block in pandoc.blocks {
//         result.blocks.push(new_block);
//         match block {
//             pandoc::P
//             }
//         }
//     }
//     result
// }

pub fn traverse_para(para: pandoc::Block::Paragraph) -> bool {
    true
}