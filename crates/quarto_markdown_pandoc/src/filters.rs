/*
 * filters.rs
 * Copyright (c) 2025 Posit, PBC
 */

use crate::pandoc::{self, AsInline, Block};

// filters are destructive and take ownership of the input

pub enum FilterReturn<T, U> {
    Unchanged(T),
    FilterResult(U, bool), // (new content, should recurse)
}

type InlineFilterFn<T> = fn(T) -> FilterReturn<T, crate::pandoc::Inlines>;
type BlockFilterFn<T> = fn(T) -> FilterReturn<T, crate::pandoc::Blocks>;
type InlineFilterField<T> = Option<InlineFilterFn<T>>;
type BlockFilterField<T> = Option<BlockFilterFn<T>>;

// Macro to generate repetitive match arms
// Macro to reduce repetition in filter logic
macro_rules! handle_inline_filter {
    ($variant:ident, $value:ident, $filter_field:ident, $filter:expr) => {
        if let Some(f) = $filter.$filter_field {
            return inlines_apply_and_maybe_recurse($value, f, $filter);
        } else if let Some(f) = $filter.inline {
            return inlines_apply_and_maybe_recurse($value.as_inline(), f, $filter);
        } else {
            vec![traverse_inline_structure(crate::pandoc::Inline::$variant($value), $filter)]
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
            vec![traverse_block_structure(crate::pandoc::Block::$variant($value), $filter)]
        }
    };
}

trait InlineFilterableStructure {
    fn filter_structure(self, filter: &Filter) -> pandoc::Inline;
}

macro_rules! impl_inline_filterable_terminal {
    ($($variant:ident),*) => {
        $(
            impl InlineFilterableStructure for pandoc::$variant {
                fn filter_structure(self, _: &Filter) -> pandoc::Inline {
                    pandoc::Inline::$variant(self)
                }
            }
        )*
    };
}
impl_inline_filterable_terminal!(
    Str, Code, Space, SoftBreak, LineBreak, Math, RawInline,
    Shortcode, NoteReference
);

macro_rules! impl_inline_filterable_simple {
    ($($variant:ident),*) => {
        $(
            impl InlineFilterableStructure for pandoc::$variant {
                fn filter_structure(self, filter: &Filter) -> pandoc::Inline {
                    pandoc::Inline::$variant(pandoc::$variant {
                        content: topdown_traverse_inlines(self.content, filter),
                        ..self
                    })
                }
            }
        )*
    };
}

impl_inline_filterable_simple!(
    Emph, Underline, Strong, Strikeout, Superscript, Subscript,
    SmallCaps, Quoted, Link, Image, Span
);

impl InlineFilterableStructure for pandoc::Note {
    fn filter_structure(self, filter: &Filter) -> pandoc::Inline {
        pandoc::Inline::Note(pandoc::Note {
            content: topdown_traverse_blocks(self.content, filter),
        })
    }
}

impl InlineFilterableStructure for pandoc::Cite {
    fn filter_structure(self, filter: &Filter) -> pandoc::Inline {
        pandoc::Inline::Cite(pandoc::Cite {
            citations: self.citations.into_iter().map(|cit| {
                pandoc::Citation {
                    id: cit.id,
                    prefix: topdown_traverse_inlines(cit.prefix, filter),
                    suffix: topdown_traverse_inlines(cit.suffix, filter),
                    mode: cit.mode,
                    note_num: cit.note_num,
                    hash: cit.hash,
                }
            }).collect(),
            content: topdown_traverse_inlines(self.content, filter),
        })
    }
}

impl InlineFilterableStructure for pandoc::Inline {
    fn filter_structure(self, filter: &Filter) -> pandoc::Inline {
        traverse_inline_structure(self, filter)
    }
}
trait BlockFilterableStructure {
    fn filter_structure(self, filter: &Filter) -> pandoc::Block;
}

macro_rules! impl_block_filterable_terminal {
    ($($variant:ident),*) => {
        $(
            impl BlockFilterableStructure for pandoc::$variant {
                fn filter_structure(self, _: &Filter) -> pandoc::Block {
                    pandoc::Block::$variant(self)
                }
            }
        )*
    };
}
impl_block_filterable_terminal!(
    CodeBlock, RawBlock, HorizontalRule
);

macro_rules! impl_block_filterable_simple {
    ($($variant:ident),*) => {
        $(
            impl BlockFilterableStructure for pandoc::$variant {
                fn filter_structure(self, filter: &Filter) -> pandoc::Block {
                    pandoc::Block::$variant(pandoc::$variant {
                        content: topdown_traverse_blocks(self.content, filter),
                        ..self
                    })
                }
            }
        )*
    };
}
impl_block_filterable_simple!(
    BlockQuote, Div
);

impl BlockFilterableStructure for pandoc::Paragraph {
    fn filter_structure(self, filter: &Filter) -> pandoc::Block {
        pandoc::Block::Paragraph(pandoc::Paragraph {
            content: topdown_traverse_inlines(self.content, filter),
            ..self
        })
    }
}

impl BlockFilterableStructure for pandoc::Plain {
    fn filter_structure(self, filter: &Filter) -> pandoc::Block {
        pandoc::Block::Plain(pandoc::Plain {
            content: topdown_traverse_inlines(self.content, filter),
            ..self
        })
    }
}

impl BlockFilterableStructure for pandoc::LineBlock {
    fn filter_structure(self, filter: &Filter) -> pandoc::Block {
        pandoc::Block::LineBlock(pandoc::LineBlock {
            content: self.content.into_iter()
                .map(|inlines| topdown_traverse_inlines(inlines, filter))
                .collect(),
            ..self
        })
    }
}

impl BlockFilterableStructure for pandoc::OrderedList {
    fn filter_structure(self, filter: &Filter) -> pandoc::Block {
        pandoc::Block::OrderedList(pandoc::OrderedList {
            content: self.content.into_iter()
                .map(|blocks| topdown_traverse_blocks(blocks, filter))
                .collect(),
            ..self
        })
    }
}

impl BlockFilterableStructure for pandoc::BulletList {
    fn filter_structure(self, filter: &Filter) -> pandoc::Block {
        pandoc::Block::BulletList(pandoc::BulletList {
            content: self.content.into_iter()
                .map(|blocks| topdown_traverse_blocks(blocks, filter))
                .collect(),
            ..self
        })
    }
}

impl BlockFilterableStructure for pandoc::DefinitionList {
    fn filter_structure(self, filter: &Filter) -> pandoc::Block {
        pandoc::Block::DefinitionList(pandoc::DefinitionList {
            content: self.content.into_iter().map(|(term, def)| (
                topdown_traverse_inlines(term, filter), 
                def.into_iter().map(|blocks| topdown_traverse_blocks(blocks, filter)).collect()
            )).collect(),
            ..self
        })
    }
}

impl BlockFilterableStructure for pandoc::Header {
    fn filter_structure(self, filter: &Filter) -> pandoc::Block {
        pandoc::Block::Header(pandoc::Header {
            content: topdown_traverse_inlines(self.content, filter),
            ..self
        })
    }
}

impl BlockFilterableStructure for pandoc::Table {
    fn filter_structure(self, filter: &Filter) -> pandoc::Block {
        pandoc::Block::Table(pandoc::Table {
            caption: traverse_caption(self.caption, filter),
            head: pandoc::TableHead {
                rows: self.head.rows.into_iter()
                    .map(|row| traverse_row(row, filter))
                    .collect(),
                ..self.head
            },
            bodies: self.bodies.into_iter().map(|body| pandoc::TableBody {
                head: body.head.into_iter()
                    .map(|row| traverse_row(row, filter))
                    .collect(),
                body: body.body.into_iter()
                    .map(|row| traverse_row(row, filter))
                    .collect(),
                ..body
            }).collect(),
            foot: pandoc::TableFoot {
                rows: self.foot.rows.into_iter()
                    .map(|row| traverse_row(row, filter))
                    .collect(),
                ..self.foot
            },
            ..self
        })
    }
}

impl BlockFilterableStructure for pandoc::Figure {
    fn filter_structure(self, filter: &Filter) -> pandoc::Block {
        pandoc::Block::Figure(pandoc::Figure {
            caption: traverse_caption(self.caption, filter),
            content: topdown_traverse_blocks(self.content, filter),
            ..self
        })
    }
}

impl BlockFilterableStructure for pandoc::Block {
    fn filter_structure(self, filter: &Filter) -> pandoc::Block {
        traverse_block_structure(self, filter)
    }
}

#[derive(Default)]
pub struct Filter {
    pub inlines: InlineFilterField<pandoc::Inlines>,
    pub blocks: BlockFilterField<pandoc::Blocks>,

    pub inline: InlineFilterField<pandoc::Inline>,
    pub block: BlockFilterField<pandoc::Block>,

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
    pub plain: BlockFilterField<pandoc::Plain>,
    pub code_block: BlockFilterField<pandoc::CodeBlock>,
    pub raw_block: BlockFilterField<pandoc::RawBlock>,
    pub bullet_list: BlockFilterField<pandoc::BulletList>,
    pub ordered_list: BlockFilterField<pandoc::OrderedList>,
    pub block_quote: BlockFilterField<pandoc::BlockQuote>,
    pub div: BlockFilterField<pandoc::Div>,
    pub figure: BlockFilterField<pandoc::Figure>,
    pub line_block: BlockFilterField<pandoc::LineBlock>,
    pub definition_list: BlockFilterField<pandoc::DefinitionList>,
    pub header: BlockFilterField<pandoc::Header>,
    pub table: BlockFilterField<pandoc::Table>,
}

fn inlines_apply_and_maybe_recurse<T: InlineFilterableStructure>(
    item: T,
    filter_fn: InlineFilterFn<T>,
    filter: &Filter
) -> crate::pandoc::Inlines {
    match filter_fn(item) {
        FilterReturn::Unchanged(inline) => 
            vec![inline.filter_structure(filter)],
        FilterReturn::FilterResult(new_content, recurse) => {
            if !recurse {
                new_content
            } else {
                topdown_traverse_inlines(new_content, filter)
            }
        }
    }
}

fn blocks_apply_and_maybe_recurse<T: BlockFilterableStructure>(
    item: T,
    filter_fn: BlockFilterFn<T>,
    filter: &Filter
) -> crate::pandoc::Blocks {
    match filter_fn(item) {
        FilterReturn::Unchanged(block) => 
        vec![block.filter_structure(filter)],
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
        crate::pandoc::Block::Plain(plain) => {
            handle_block_filter!(Plain, plain, plain, filter)
        },
        crate::pandoc::Block::LineBlock(line_block) => {
            handle_block_filter!(LineBlock, line_block, line_block, filter)
        },
        crate::pandoc::Block::DefinitionList(def_list) => {
            handle_block_filter!(DefinitionList, def_list, definition_list, filter)
        },
        crate::pandoc::Block::Header(header) => {
            handle_block_filter!(Header, header, header, filter)
        },
        crate::pandoc::Block::Table(table) => {
            handle_block_filter!(Table, table, table, filter)
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

fn traverse_inline_nonterminal(
    inline: crate::pandoc::Inline,
    filter: &Filter
) -> crate::pandoc::Inline {
    match inline {
        crate::pandoc::Inline::Emph(e) => {
           crate::pandoc::Inline::Emph(crate::pandoc::Emph {
              content: topdown_traverse_inlines(e.content, filter)
           })
        },
        crate::pandoc::Inline::Underline(u) => {
            crate::pandoc::Inline::Underline(crate::pandoc::Underline {
                content: topdown_traverse_inlines(u.content, filter)
            })
        },
        crate::pandoc::Inline::Strong(sg) => {
            crate::pandoc::Inline::Strong(crate::pandoc::Strong {
                content: topdown_traverse_inlines(sg.content, filter)
            })
        },
        crate::pandoc::Inline::Strikeout(st) => {
            crate::pandoc::Inline::Strikeout(crate::pandoc::Strikeout {
                content: topdown_traverse_inlines(st.content, filter)
            })
        },
        crate::pandoc::Inline::Superscript(sp) => {
            crate::pandoc::Inline::Superscript(crate::pandoc::Superscript
                { content: topdown_traverse_inlines(sp.content, filter) }
            )
        },
        crate::pandoc::Inline::Subscript(sb) => {
            crate::pandoc::Inline::Subscript(crate::pandoc::Subscript {
                content: topdown_traverse_inlines(sb.content, filter)
            })
        },
        crate::pandoc::Inline::SmallCaps(sc) => {
            crate::pandoc::Inline::SmallCaps(crate::pandoc::SmallCaps {
                content: topdown_traverse_inlines(sc.content, filter)
            })
        },
        crate::pandoc::Inline::Quoted(q) => {
            crate::pandoc::Inline::Quoted(crate::pandoc::Quoted {
                quote_type: q.quote_type,
                content: topdown_traverse_inlines(q.content, filter),
            })
        },
        crate::pandoc::Inline::Cite(c) => {
            crate::pandoc::Inline::Cite(crate::pandoc::Cite {
                citations: c.citations.into_iter().map(|cit| {
                    crate::pandoc::Citation {
                        id: cit.id,
                        prefix: topdown_traverse_inlines(cit.prefix, filter),
                        suffix: topdown_traverse_inlines(cit.suffix, filter),
                        mode: cit.mode,
                        note_num: cit.note_num,
                        hash: cit.hash,
                    }
                }).collect(),
                content: topdown_traverse_inlines(c.content, filter),
            })
        },
        crate::pandoc::Inline::Link(l) => {
            crate::pandoc::Inline::Link(crate::pandoc::Link {
                attr: l.attr,
                target: l.target,
                content: topdown_traverse_inlines(l.content, filter),
            })
        },
        crate::pandoc::Inline::Image(i) => {
            crate::pandoc::Inline::Image(crate::pandoc::Image {
                attr: i.attr,
                target: i.target,
                content: topdown_traverse_inlines(i.content, filter),
            })
        },
        crate::pandoc::Inline::Note(note) => {
            crate::pandoc::Inline::Note(crate::pandoc::Note {
                content: topdown_traverse_blocks(note.content, filter),
            })
        },
        crate::pandoc::Inline::Span(span) => {
            crate::pandoc::Inline::Span(crate::pandoc::Span {
                attr: span.attr,
                content: topdown_traverse_inlines(span.content, filter),
            })
        },

        _ => panic!("Unsupported inline type: {:?}", inline),
    }
}

pub fn traverse_inline_structure(
    inline: crate::pandoc::Inline,
    filter: &Filter
) -> crate::pandoc::Inline {
    match &inline {
        // terminal inline types
        crate::pandoc::Inline::Str(_) => inline,
        crate::pandoc::Inline::Code(_) => inline,
        crate::pandoc::Inline::Space(_) => inline,
        crate::pandoc::Inline::SoftBreak(_) => inline,
        crate::pandoc::Inline::LineBreak(_) => inline,
        crate::pandoc::Inline::Math(_) => inline,
        crate::pandoc::Inline::RawInline(_) => inline,
        // extensions
        crate::pandoc::Inline::Shortcode(_) => inline,
        crate::pandoc::Inline::NoteReference(_) => inline,
        _ => traverse_inline_nonterminal(inline, filter)
    }
}

fn traverse_blocks_vec_nonterminal(
    blocks_vec: Vec<crate::pandoc::Blocks>,
    filter: &Filter
) -> Vec<crate::pandoc::Blocks> {
    blocks_vec
        .into_iter()
        .map(|blocks| topdown_traverse_blocks(blocks, filter))
        .collect()
}

fn traverse_caption(caption: crate::pandoc::Caption, filter: &Filter) -> crate::pandoc::Caption {
    crate::pandoc::Caption {
        short: caption.short.map(|short| {
            topdown_traverse_inlines(short, filter)
        }),
        long: caption.long.map(|long| {
            topdown_traverse_blocks(long, filter)
        })
    }
}

fn traverse_row(row: crate::pandoc::Row, filter: &Filter) -> crate::pandoc::Row {
    crate::pandoc::Row {
        cells: row.cells
            .into_iter()
            .map(|cell| {
                crate::pandoc::Cell {
                    content: topdown_traverse_blocks(cell.content, filter),
                    ..cell
                }
            })
            .collect(),
        ..row
    }
}

fn traverse_rows(
    rows: Vec<crate::pandoc::Row>,
    filter: &Filter
) -> Vec<crate::pandoc::Row> {
    rows.into_iter()
        .map(|row| traverse_row(row, filter))
        .collect()
}

fn traverse_block_nonterminal(
    block: crate::pandoc::Block,
    filter: &Filter
) -> crate::pandoc::Block {
    match block {
        crate::pandoc::Block::Plain(plain) => {
            crate::pandoc::Block::Plain(crate::pandoc::Plain {
                content: topdown_traverse_inlines(plain.content, filter),
                ..plain
            })
        }
        crate::pandoc::Block::Paragraph(para) => {
            crate::pandoc::Block::Paragraph(crate::pandoc::Paragraph {
                content: topdown_traverse_inlines(para.content, filter),
                ..para
            })
        },
        crate::pandoc::Block::LineBlock(line_block) => {
            crate::pandoc::Block::LineBlock(crate::pandoc::LineBlock {
                content: line_block.content
                    .into_iter()
                    .map(|line| topdown_traverse_inlines(line, filter))
                    .collect(),
                ..line_block
            })
        },
        crate::pandoc::Block::BlockQuote(quote) => {
            crate::pandoc::Block::BlockQuote(crate::pandoc::BlockQuote {
                content: topdown_traverse_blocks(quote.content, filter),
                ..quote
            })
        },
        crate::pandoc::Block::OrderedList(list) => {
            crate::pandoc::Block::OrderedList(crate::pandoc::OrderedList {
                content: traverse_blocks_vec_nonterminal(
                    list.content, filter),
                ..list
            })
        },
        crate::pandoc::Block::BulletList(list) => {
            crate::pandoc::Block::BulletList(crate::pandoc::BulletList {
                content: traverse_blocks_vec_nonterminal(
                    list.content, filter),
                ..list
            })
        },
        crate::pandoc::Block::DefinitionList(list) => {
            crate::pandoc::Block::DefinitionList(crate::pandoc::DefinitionList {
                content: list.content
                    .into_iter()
                    .map(|(term, def)| (
                        topdown_traverse_inlines(term, filter), 
                        traverse_blocks_vec_nonterminal(def, filter)))
                    .collect(),
                ..list
            })
        },
        crate::pandoc::Block::Header(header) => {
            crate::pandoc::Block::Header(crate::pandoc::Header {
                content: topdown_traverse_inlines(header.content, filter),
                ..header
            })
        },
        crate::pandoc::Block::Table(table) => {
            crate::pandoc::Block::Table(crate::pandoc::Table {
                caption: traverse_caption(table.caption, filter),
                head: crate::pandoc::TableHead {
                    rows: traverse_rows(table.head.rows, filter),
                    ..table.head
                },
                bodies: table.bodies
                    .into_iter()
                    .map(|table_body| {
                        crate::pandoc::TableBody {
                            head: traverse_rows(table_body.head, filter),
                            body: traverse_rows(table_body.body, filter),
                            ..table_body
                        }
                    })
                    .collect(),
                foot: crate::pandoc::TableFoot {
                    rows: traverse_rows(table.foot.rows, filter),
                    ..table.foot
                },
                ..table
            })
        },
        crate::pandoc::Block::Figure(figure) => {
            crate::pandoc::Block::Figure(crate::pandoc::Figure {
                caption: traverse_caption(figure.caption, filter),
                content: topdown_traverse_blocks(figure.content, filter),
                ..figure
            })
        },
        crate::pandoc::Block::Div(div) => {
            crate::pandoc::Block::Div(crate::pandoc::Div {
                content: topdown_traverse_blocks(div.content, filter),
                ..div
            })
        },
        _ => {
            panic!("Unsupported block type: {:?}", block);
        }
    }
}
pub fn traverse_block_structure(
    block: crate::pandoc::Block,
    filter: &Filter
) -> crate::pandoc::Block {
    match &block {
        // terminal block types
        crate::pandoc::Block::CodeBlock(_) => block,
        crate::pandoc::Block::RawBlock(_) => block,
        crate::pandoc::Block::HorizontalRule(_) => block,
        _ => traverse_block_nonterminal(block, filter)
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