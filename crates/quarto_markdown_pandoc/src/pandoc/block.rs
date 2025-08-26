/*
 * block.rs
 * Copyright (c) 2025 Posit, PBC
 */

use crate::impl_source_location;
use crate::pandoc::Meta;
use crate::pandoc::attr::Attr;
use crate::pandoc::caption::Caption;
use crate::pandoc::inline::Inlines;
use crate::pandoc::list::ListAttributes;
use crate::pandoc::location::Range;
use crate::pandoc::location::SourceLocation;
use crate::pandoc::location::node_location;
use crate::pandoc::table::Table;

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Plain(Plain),
    Paragraph(Paragraph),
    LineBlock(LineBlock),
    CodeBlock(CodeBlock),
    RawBlock(RawBlock),
    BlockQuote(BlockQuote),
    OrderedList(OrderedList),
    BulletList(BulletList),
    DefinitionList(DefinitionList),
    Header(Header),
    HorizontalRule(HorizontalRule),
    Table(Table),
    Figure(Figure),
    Div(Div),
    // quarto extensions
    BlockMetadata(MetaBlock),
}

pub type Blocks = Vec<Block>;

#[derive(Debug, Clone, PartialEq)]
pub struct Plain {
    pub content: Inlines,

    pub filename: Option<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph {
    pub content: Inlines,

    pub filename: Option<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineBlock {
    pub content: Vec<Inlines>,

    pub filename: Option<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock {
    pub attr: Attr,
    pub text: String,

    pub filename: Option<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawBlock {
    pub format: String,
    pub text: String,

    pub filename: Option<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockQuote {
    pub content: Blocks,

    pub filename: Option<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderedList {
    pub attr: ListAttributes,
    pub content: Vec<Blocks>,

    pub filename: Option<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BulletList {
    pub content: Vec<Blocks>,

    pub filename: Option<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefinitionList {
    pub content: Vec<(Inlines, Vec<Blocks>)>,

    pub filename: Option<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub level: usize,
    pub attr: Attr,
    pub content: Inlines,

    pub filename: Option<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HorizontalRule {
    pub filename: Option<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Figure {
    pub attr: Attr,
    pub caption: Caption,
    pub content: Blocks,

    pub filename: Option<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Div {
    pub attr: Attr,
    pub content: Blocks,

    pub filename: Option<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetaBlock {
    pub meta: Meta,

    pub filename: Option<String>,
    pub range: Range,
}

impl_source_location!(
    // blocks
    Plain,
    Paragraph,
    LineBlock,
    CodeBlock,
    RawBlock,
    BlockQuote,
    OrderedList,
    BulletList,
    DefinitionList,
    Header,
    HorizontalRule,
    Table,
    Figure,
    Div,
    // quarto extensions
    MetaBlock
);

fn make_block_leftover(node: &tree_sitter::Node, input_bytes: &[u8]) -> Block {
    let text = node.utf8_text(input_bytes).unwrap().to_string();
    Block::RawBlock(RawBlock {
        format: "quarto-internal-leftover".to_string(),
        text,
        filename: None,
        range: node_location(node),
    })
}
