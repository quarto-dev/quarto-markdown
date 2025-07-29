/*
 * filters.rs
 * Copyright (c) 2025 Posit, PBC
 */

use crate::pandoc;

// filters are destructive and take ownership of the input

pub enum FilterReturn<T> {
    Unchanged(T),
    FilterResult(T, bool), // (new content, should recurse)
}

type InlineFilterFn<T> = fn(T) -> FilterReturn<crate::pandoc::Inlines>;
type BlockFilterFn<T> = fn(T) -> FilterReturn<crate::pandoc::Blocks>;
type InlineFilterField<T> = Option<InlineFilterFn<T>>;
type BlockFilterField<T> = Option<BlockFilterFn<T>>;

// Macro to generate repetitive match arms
// Macro to reduce repetition in filter logic
macro_rules! handle_inline_filter {
    ($variant:ident, $value:ident, $filter_field:ident, $filter:expr) => {
        if let Some(f) = $filter.$filter_field {
            return inlines_apply_and_maybe_recurse($value, f, $filter);
        } else if let Some(f) = $filter.inline {
            return inlines_apply_and_maybe_recurse(crate::pandoc::Inline::$variant($value), f, $filter);
        } else {
            vec![crate::pandoc::Inline::$variant($value)]
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
            vec![crate::pandoc::Block::$variant($value)]
        }
    };
    ($variant:ident, $value:ident, $filter_field:ident, $filter:expr, $default:expr) => {
        if let Some(f) = $filter.$filter_field {
            return blocks_apply_and_maybe_recurse($value, f, $filter);
        } else if let Some(f) = $filter.block {
            return blocks_apply_and_maybe_recurse(
                crate::pandoc::Block::$variant($value), f, $filter);
        } else {
            $default
        }
    };
}

#[derive(Default)]
pub struct Filter {
    pub inlines: InlineFilterField<pandoc::Inlines>,
    pub inline: InlineFilterField<pandoc::Inline>,

    pub block: BlockFilterField<pandoc::Block>,
    pub blocks: BlockFilterField<pandoc::Blocks>,

    pub str: InlineFilterField<pandoc::Str>,
    pub emph: InlineFilterField<pandoc::Emph>,
    pub underline: InlineFilterField<pandoc::Underline>,
    pub strong: InlineFilterField<pandoc::Strong>,
    pub strikeout: InlineFilterField<pandoc::Strikeout>,
    pub superscript: InlineFilterField<pandoc::Superscript>,
    pub subscript: InlineFilterField<pandoc::Subscript>,
    pub small_caps: InlineFilterField<pandoc::SmallCaps>,
    pub quoted: InlineFilterField<pandoc::Quoted>,
    pub cite: InlineFilterField<pandoc::Cite>,
    pub code: InlineFilterField<pandoc::Code>,
    pub space: InlineFilterField<pandoc::Space>,
    pub soft_break: InlineFilterField<pandoc::SoftBreak>,
    pub line_break: InlineFilterField<pandoc::LineBreak>,
    pub math: InlineFilterField<pandoc::Math>,
    pub raw_inline: InlineFilterField<pandoc::RawInline>,
    pub link: InlineFilterField<pandoc::Link>,
    pub image: InlineFilterField<pandoc::Image>,
    pub note: InlineFilterField<pandoc::Note>,
    pub span: InlineFilterField<pandoc::Span>,
    pub shortcode: InlineFilterField<pandoc::Shortcode>,
    pub note_reference: InlineFilterField<pandoc::NoteReference>,

    pub paragraph: BlockFilterField<pandoc::Paragraph>,
    pub code_block: BlockFilterField<pandoc::CodeBlock>,
    pub raw_block: BlockFilterField<pandoc::RawBlock>,
    pub bullet_list: BlockFilterField<pandoc::BulletList>,
    pub ordered_list: BlockFilterField<pandoc::OrderedList>,
    pub block_quote: BlockFilterField<pandoc::BlockQuote>,
    pub div: BlockFilterField<pandoc::Div>,
}

fn inlines_apply_and_maybe_recurse<T>(
    item: T,
    filter_fn: InlineFilterFn<T>,
    filter: &Filter
) -> crate::pandoc::Inlines {
    match filter_fn(item) {
        FilterReturn::Unchanged(inlines) => inlines,
        FilterReturn::FilterResult(new_content, recurse) => {
            if !recurse {
                new_content
            } else {
                topdown_traverse_inlines(new_content, filter)
            }
        }
    }
}

fn blocks_apply_and_maybe_recurse<T>(
    item: T,
    filter_fn: BlockFilterFn<T>,
    filter: &Filter
) -> crate::pandoc::Blocks {
    match filter_fn(item) {
        FilterReturn::Unchanged(blocks) => blocks,
        FilterReturn::FilterResult(new_content, recurse) => {
            if !recurse {
                new_content
            } else {
                topdown_traverse_blocks(new_content, filter)
            }
        }
    }
}

pub fn topdown_traverse_inline(inline: crate::pandoc::Inline, filter: &Filter) -> crate::pandoc::Inlines {
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

pub fn topdown_traverse_block(block: crate::pandoc::Block, filter: &Filter) -> crate::pandoc::Blocks {
    match block {
        crate::pandoc::Block::Paragraph(para) => {
            handle_block_filter!(Paragraph, para, paragraph, filter, 
                vec![crate::pandoc::Block::Paragraph(
                    crate::pandoc::Paragraph {
                        content: topdown_traverse_inlines(para.content, filter),
                        filename: para.filename,
                        range: para.range,
                    })])
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
            result.extend(topdown_traverse_inline(inline, filter));
        }
        result
    }
    match filter.inlines {
        None => walk_vec(vec, filter),
        Some(f) => {
            match f(vec) {
                FilterReturn::Unchanged(inlines) => walk_vec(inlines, filter),
                FilterReturn::FilterResult(new_content, recurse) => {
                    if !recurse {
                        return new_content;
                    }
                    walk_vec(new_content, filter)
                }
            }
        },
    }
}

pub fn topdown_traverse_blocks(vec: crate::pandoc::Blocks, filter: &Filter) -> crate::pandoc::Blocks {
    fn walk_vec(
        vec: crate::pandoc::Blocks,
        filter: &Filter
    ) -> crate::pandoc::Blocks {
        let mut result = vec![];
        for block in vec {
            result.extend(topdown_traverse_block(block, filter));
        }
        result
    }
    match filter.blocks {
        None => walk_vec(vec, filter),
        Some(f) => {
            match f(vec) {
                FilterReturn::Unchanged(blocks) => walk_vec(blocks, filter),
                FilterReturn::FilterResult(new_content, recurse) => {
                    if !recurse {
                        return new_content;
                    }
                    walk_vec(new_content, filter)
                }
            }
        },
    }
}

pub fn topdown_traverse(doc: pandoc::Pandoc, filter: &Filter) -> pandoc::Pandoc {
    pandoc::Pandoc {
        blocks: topdown_traverse_blocks(doc.blocks, filter),
        // TODO: handle meta
    }
}