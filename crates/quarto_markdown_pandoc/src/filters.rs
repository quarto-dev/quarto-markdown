/*
 * filters.rs
 * Copyright (c) 2025 Posit, PBC
 */

use crate::pandoc;

// filters are destructive and take ownership of the input

type InlineFilterFn<T> = Option<fn(T) -> Option<(Vec<pandoc::Inline>, bool)>>;
type BlockFilterFn<T> = Option<fn(T) -> Option<(Vec<pandoc::Block>, bool)>>;

// Macro to generate repetitive match arms
// Macro to reduce repetition in filter logic
macro_rules! handle_inline_filter {
    ($variant:ident, $value:ident, $filter_field:ident, $filter:expr) => {
        if let Some(f) = $filter.$filter_field {
            return inlines_apply_and_maybe_recurse($value, f, $filter);
        } else if let Some(f) = $filter.inline {
            return inlines_apply_and_maybe_recurse(crate::pandoc::Inline::$variant($value), f, $filter);
        } else {
            None
        }
    };
}

macro_rules! handle_block_filter {
    ($variant:ident, $value:ident, $filter_field:ident, $filter:expr) => {
        if let Some(f) = $filter.$filter_field {
            return blocks_apply_and_maybe_recurse($value, f, $filter);
        } else if let Some(f) = $filter.block {
            return blocks_apply_and_maybe_recurse(
                crate::pandoc::Block::$variant($value), f, $filter);
        } else {
            None
        }
    };
}

#[derive(Default)]
pub struct Filter {
    pub inlines: InlineFilterFn<pandoc::Inlines>,
    pub inline: InlineFilterFn<pandoc::Inline>,

    pub block: BlockFilterFn<pandoc::Block>,
    pub blocks: BlockFilterFn<pandoc::Blocks>,

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
    pub note_reference: InlineFilterFn<pandoc::NoteReference>,

    pub paragraph: BlockFilterFn<pandoc::Paragraph>,
    pub code_block: BlockFilterFn<pandoc::CodeBlock>,
    pub raw_block: BlockFilterFn<pandoc::RawBlock>,
    pub bullet_list: BlockFilterFn<pandoc::BulletList>,
    pub ordered_list: BlockFilterFn<pandoc::OrderedList>,
    pub block_quote: BlockFilterFn<pandoc::BlockQuote>,
    pub div: BlockFilterFn<pandoc::Div>,
    pub figure: BlockFilterFn<pandoc::Figure>,
}

fn inlines_apply_and_maybe_recurse<T>(
    item: T,
    filter_fn: fn(T) -> Option<(crate::pandoc::Inlines, bool)>,
    filter: &Filter
) -> Option<crate::pandoc::Inlines> {
    match filter_fn(item) {
        None => None,
        Some((new_content, recurse)) => {
            if !recurse {
                Some(new_content)
            } else {
                Some(topdown_traverse_inlines(new_content, filter))
            }
        }
    }
}

fn blocks_apply_and_maybe_recurse<T>(
    item: T,
    filter_fn: fn(T) -> Option<(crate::pandoc::Blocks, bool)>,
    filter: &Filter
) -> Option<crate::pandoc::Blocks> {
    match filter_fn(item) {
        None => None,
        Some((new_content, recurse)) => {
            if !recurse {
                Some(new_content)
            } else {
                Some(topdown_traverse_blocks(new_content, filter))
            }
        }
    }
}

pub fn topdown_traverse_inline(inline: crate::pandoc::Inline, filter: &Filter) -> Option<crate::pandoc::Inlines> {
    match inline {
        crate::pandoc::Inline::Str(s) => {
            handle_inline_filter!(Str, s, str, filter)
        },
        crate::pandoc::Inline::Emph(e) => {
            handle_inline_filter!(Emph, e, emph, filter)
        },
        crate::pandoc::Inline::Underline(u) => {
            handle_inline_filter!(Underline, u, underline, filter)
        },
        crate::pandoc::Inline::Strong(sg) => {
            handle_inline_filter!(Strong, sg, strong, filter)
        },
        crate::pandoc::Inline::Strikeout(st) => {
            handle_inline_filter!(Strikeout, st, strikeout, filter)
        },
        crate::pandoc::Inline::Superscript(sp) => {
            handle_inline_filter!(Superscript, sp, superscript, filter)
        },
        crate::pandoc::Inline::Subscript(sb) => {
            handle_inline_filter!(Subscript, sb, subscript, filter)
        },
        crate::pandoc::Inline::SmallCaps(sc) => {
            handle_inline_filter!(SmallCaps, sc, small_caps, filter)
        },
        crate::pandoc::Inline::Quoted(q) => {
            handle_inline_filter!(Quoted, q, quoted, filter)
        },
        crate::pandoc::Inline::Cite(c) => {
            handle_inline_filter!(Cite, c, cite, filter)
        },
        crate::pandoc::Inline::Code(co) => {
            handle_inline_filter!(Code, co, code, filter)
        },
        crate::pandoc::Inline::Space(sp) => {
            handle_inline_filter!(Space, sp, space, filter)
        },
        crate::pandoc::Inline::SoftBreak(sb) => {
            handle_inline_filter!(SoftBreak, sb, soft_break, filter)
        },
        crate::pandoc::Inline::LineBreak(lb) => {
            handle_inline_filter!(LineBreak, lb, line_break, filter)
        },
        crate::pandoc::Inline::Math(m) => {
            handle_inline_filter!(Math, m, math, filter)
        },
        crate::pandoc::Inline::RawInline(ri) => {
            handle_inline_filter!(RawInline, ri, raw_inline, filter)
        },
        crate::pandoc::Inline::Link(l) => {
            handle_inline_filter!(Link, l, link, filter)
        },
        crate::pandoc::Inline::Image(i) => {
            handle_inline_filter!(Image, i, image, filter)
        },
        crate::pandoc::Inline::Note(note) => {
            handle_inline_filter!(Note, note, note, filter)
        },
        crate::pandoc::Inline::Span(span) => {
            handle_inline_filter!(Span, span, span, filter)
        },
        // quarto extensions
        crate::pandoc::Inline::Shortcode(shortcode) => {
            handle_inline_filter!(Shortcode, shortcode, shortcode, filter)
        },
        crate::pandoc::Inline::NoteReference(note_ref) => {
            handle_inline_filter!(NoteReference, note_ref, note_reference, filter)
        },
    }
}

pub fn topdown_traverse_block(block: crate::pandoc::Block, filter: &Filter) -> Option<crate::pandoc::Blocks> {
    match block {
        crate::pandoc::Block::Paragraph(para) => {
            handle_block_filter!(Paragraph, para, paragraph, filter)
        },
        crate::pandoc::Block::CodeBlock(code) => {
            handle_block_filter!(CodeBlock, code, code_block, filter)
        },
        crate::pandoc::Block::RawBlock(raw) => {
            handle_block_filter!(RawBlock, raw, raw_block, filter)
        },
        crate::pandoc::Block::BulletList(list) => {
            handle_block_filter!(BulletList, list, bullet_list, filter)
        },
        crate::pandoc::Block::OrderedList(list) => {
            handle_block_filter!(OrderedList, list, ordered_list, filter)
        },
        crate::pandoc::Block::BlockQuote(quote) => {
            handle_block_filter!(BlockQuote, quote, block_quote, filter)
        },
        crate::pandoc::Block::Div(div) => {
            handle_block_filter!(Div, div, div, filter)
        },
        crate::pandoc::Block::Figure(figure) => {
            handle_block_filter!(Figure, figure, figure, filter)
        },
        _ => panic!("Unsupported block type: {:?}", block),
    }
}

pub fn topdown_traverse_inlines(vec: crate::pandoc::Inlines, filter: &Filter) -> crate::pandoc::Inlines {
    fn walk_vec(
        vec: crate::pandoc::Inlines,
        filter: &Filter
    ) -> crate::pandoc::Inlines {
        let mut result = vec![];
        for inline in vec {
            match topdown_traverse_inline(inline, filter) {
                Some(inlines) => result.extend(inlines),
                None => result.push(inline), // Skip if the filter returns None
            }
        }
        result
    }
    if let Some(f) = filter.inlines {
        let (inner_result, recurse) = f(vec);
        if !recurse {
            return inner_result;
        }
        walk_vec(inner_result, filter)
    } else {
        walk_vec(vec, filter)
    }
}

pub fn topdown_traverse_blocks(vec: crate::pandoc::Blocks, filter: &Filter) -> crate::pandoc::Blocks {
    if let Some(f) = filter.blocks {
        let (inner_result, recurse) = f(vec);
        if !recurse {
            return inner_result;
        } else {
            return topdown_traverse_blocks(inner_result, filter);
        }
    }
    let mut result = vec![];
    for block in vec {
        result.extend(topdown_traverse_block(block, filter));
    }
    result
}

pub fn topdown_traverse(doc: pandoc::Pandoc, filter: &Filter) -> pandoc::Pandoc {
    pandoc::Pandoc {
        blocks: topdown_traverse_blocks(doc.blocks, filter),
        // TODO: handle meta
    }
}