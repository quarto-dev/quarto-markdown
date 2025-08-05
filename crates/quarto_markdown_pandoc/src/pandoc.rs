/*
 * pandoc.rs
 * Copyright (c) 2025 Posit, PBC
 */

/*
 * A data structure that mimics Pandoc's `data Pandoc` type.
 * This is used to represent the parsed structure of a Quarto Markdown document.
 */

use core::panic;
use regex::Regex;
use std::collections::HashMap;
use std::io::Write;

use crate::errors;
use crate::filters::{
    Filter, FilterReturn::FilterResult, FilterReturn::Unchanged, topdown_traverse,
};
use crate::traversals::bottomup_traverse_concrete_tree;

////////////////////////////////////////////////////////////////////////////////////////////////////

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
}

#[derive(Debug, Clone, PartialEq)]
pub enum Inline {
    Str(Str),
    Emph(Emph),
    Underline(Underline),
    Strong(Strong),
    Strikeout(Strikeout),
    Superscript(Superscript),
    Subscript(Subscript),
    SmallCaps(SmallCaps),
    Quoted(Quoted),
    Cite(Cite),
    Code(Code),
    Space(Space),
    SoftBreak(SoftBreak),
    LineBreak(LineBreak),
    Math(Math),
    RawInline(RawInline),
    Link(Link),
    Image(Image),
    Note(Note),
    Span(Span),

    // quarto extensions
    // after desugaring, these nodes should not appear in a document
    Shortcode(Shortcode),
    NoteReference(NoteReference),
    // this is used to represent commonmark attributes in the document in places
    // where they are not directly attached to a block, like in headings and tables
    Attr(Attr),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pandoc {
    pub blocks: Vec<Block>, // eventually, meta:
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Source location tracking

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    pub offset: usize,
    pub row: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    pub start: Location,
    pub end: Location,
}

pub trait SourceLocation {
    fn filename(&self) -> Option<String>;
    fn range(&self) -> Range;
}

fn node_location(node: &tree_sitter::Node) -> Range {
    let start = node.start_position();
    let end = node.end_position();
    Range {
        start: Location {
            offset: node.start_byte(),
            row: start.row,
            column: start.column,
        },
        end: Location {
            offset: node.end_byte(),
            row: end.row,
            column: end.column,
        },
    }
}

fn empty_range() -> Range {
    Range {
        start: Location {
            offset: 0,
            row: 0,
            column: 0,
        },
        end: Location {
            offset: 0,
            row: 0,
            column: 0,
        },
    }
}

fn empty_attr() -> Attr {
    ("".to_string(), vec![], HashMap::new())
}

pub type Attr = (String, Vec<String>, HashMap<String, String>);

pub type Blocks = Vec<Block>;
pub type Inlines = Vec<Inline>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ListNumberStyle {
    Default,
    Decimal,
    LowerRoman,
    UpperRoman,
    LowerAlpha,
    UpperAlpha,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ListNumberDelim {
    Default,
    Period,
    OneParen,
    TwoParens,
}

pub type ListAttributes = (usize, ListNumberStyle, ListNumberDelim);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Default,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ColWidth {
    Default,
    Percentage(f64),
}

pub type ColSpec = (Alignment, ColWidth);

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub attr: Attr,
    pub cells: Vec<Cell>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableHead {
    pub attr: Attr,
    pub rows: Vec<Row>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableBody {
    pub attr: Attr,
    pub rowhead_columns: usize,
    pub head: Vec<Row>,
    pub body: Vec<Row>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableFoot {
    pub attr: Attr,
    pub rows: Vec<Row>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Caption {
    pub short: Option<Inlines>,
    pub long: Option<Blocks>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub attr: Attr,
    pub alignment: Alignment,
    pub row_span: usize,
    pub col_span: usize,
    pub content: Blocks,
}

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
pub struct Table {
    pub attr: Attr,
    pub caption: Caption,
    pub colspec: Vec<ColSpec>,
    pub head: TableHead,
    pub bodies: Vec<TableBody>,
    pub foot: TableFoot,

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuoteType {
    SingleQuote,
    DoubleQuote,
}

pub type Target = (String, String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MathType {
    InlineMath,
    DisplayMath,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Str {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Emph {
    pub content: Inlines,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Underline {
    pub content: Inlines,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Strong {
    pub content: Inlines,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Strikeout {
    pub content: Inlines,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Superscript {
    pub content: Inlines,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Subscript {
    pub content: Inlines,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SmallCaps {
    pub content: Inlines,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Quoted {
    pub quote_type: QuoteType,
    pub content: Inlines,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cite {
    pub citations: Vec<Citation>,
    pub content: Inlines,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Code {
    pub attr: Attr,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Math {
    pub math_type: MathType,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawInline {
    pub format: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    pub attr: Attr,
    pub content: Inlines,
    pub target: Target,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub attr: Attr,
    pub content: Inlines,
    pub target: Target,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub content: Blocks,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub attr: Attr,
    pub content: Inlines,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Space {
    pub filename: Option<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineBreak {
    pub filename: Option<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SoftBreak {
    pub filename: Option<String>,
    pub range: Range,
}

macro_rules! impl_source_location {
    ($($type:ty),*) => {
        $(
            impl SourceLocation for $type {
                fn filename(&self) -> Option<String> {
                    self.filename.clone()
                }

                fn range(&self) -> Range {
                    self.range.clone()
                }
            }
        )*
    };
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
    // inlines
    Space,
    LineBreak,
    SoftBreak
);

#[derive(Debug, Clone, PartialEq)]
pub enum ShortcodeArg {
    String(String),
    Number(f64),
    Boolean(bool),
    Shortcode(Shortcode),
    KeyValue(HashMap<String, ShortcodeArg>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Shortcode {
    pub is_escaped: bool,
    pub name: String,
    pub positional_args: Vec<ShortcodeArg>,
    pub keyword_args: HashMap<String, ShortcodeArg>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NoteReference {
    pub id: String,
    pub range: Range,
}

pub trait AsInline {
    fn as_inline(self) -> Inline;
}
macro_rules! impl_as_inline {
    ($($type:ident),*) => {
        $(
            impl AsInline for $type {
                fn as_inline(self) -> Inline {
                    Inline::$type(self)
                }
            }
        )*
    };
}
impl AsInline for Inline {
    fn as_inline(self) -> Inline {
        self
    }
}
impl_as_inline!(
    Str,
    Emph,
    Underline,
    Strong,
    Strikeout,
    Superscript,
    Subscript,
    SmallCaps,
    Quoted,
    Cite,
    Code,
    Space,
    SoftBreak,
    LineBreak,
    Math,
    RawInline,
    Link,
    Image,
    Note,
    Span,
    Shortcode,
    NoteReference,
    Attr
);

#[derive(Debug, Clone, PartialEq)]
pub struct Citation {
    pub id: String,
    pub prefix: Inlines,
    pub suffix: Inlines,
    pub mode: CitationMode,
    pub note_num: usize,
    pub hash: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitationMode {
    AuthorInText,
    SuppressAuthor,
    NormalCitation,
}

#[derive(Debug, Clone, PartialEq)]
enum PandocNativeIntermediate {
    IntermediatePandoc(Pandoc),
    IntermediateAttr(Attr),
    IntermediateSection(Vec<Block>),
    IntermediateBlock(Block),
    IntermediateInline(Inline),
    IntermediateInlines(Inlines),
    IntermediateBaseText(String, Range),
    IntermediateLatexInlineDelimiter(Range),
    IntermediateLatexDisplayDelimiter(Range),
    IntermediateKeyValueSpec(HashMap<String, String>),
    IntermediateRawFormat(String, Range),
    IntermediateShortcodeArg(ShortcodeArg, Range),
    IntermediateUnknown(Range),
    IntermediateListItem(Blocks, Range, Option<ListAttributes>),
    IntermediateOrderedListMarker(usize, Range),
    IntermediateMetadataString(String, Range),
    IntermediateCell(Cell),
    IntermediateRow(Row),
    IntermediatePipeTableDelimiterCell(Alignment),
    IntermediatePipeTableDelimiterRow(Vec<Alignment>),
    IntermediateSetextHeadingLevel(usize),
}

fn is_empty_attr(attr: &Attr) -> bool {
    attr.0.is_empty() && attr.1.is_empty() && attr.2.is_empty()
}

fn is_empty_target(target: &Target) -> bool {
    target.0.is_empty() && target.1.is_empty()
}

fn make_span_inline(attr: Attr, target: Target, content: Inlines) -> Inline {
    // non-empty targets are never Underline or SmallCaps
    if !is_empty_target(&target) {
        return Inline::Link(Link {
            attr,
            content,
            target,
        });
    }
    if attr.1.contains(&"smallcaps".to_string()) {
        let mut new_attr = attr.clone();
        new_attr.1 = new_attr
            .1
            .into_iter()
            .filter(|s| s != "smallcaps")
            .collect();
        if is_empty_attr(&new_attr) {
            return Inline::SmallCaps(SmallCaps { content });
        }
        let inner_inline = make_span_inline(new_attr, target, content);
        return Inline::SmallCaps(SmallCaps {
            content: vec![inner_inline],
        });
    } else if attr.1.contains(&"ul".to_string()) {
        let mut new_attr = attr.clone();
        new_attr.1 = new_attr.1.into_iter().filter(|s| s != "ul").collect();
        if is_empty_attr(&new_attr) {
            return Inline::Underline(Underline { content });
        }
        let inner_inline = make_span_inline(new_attr, target, content);
        return Inline::Underline(Underline {
            content: vec![inner_inline],
        });
    } else if attr.1.contains(&"underline".to_string()) {
        let mut new_attr = attr.clone();
        new_attr.1 = new_attr
            .1
            .into_iter()
            .filter(|s| s != "underline")
            .collect();
        if is_empty_attr(&new_attr) {
            return Inline::Underline(Underline { content });
        }
        let inner_inline = make_span_inline(new_attr, target, content);
        return Inline::Underline(Underline {
            content: vec![inner_inline],
        });
    }

    return Inline::Span(Span { attr, content });
}

fn make_cite_inline(attr: Attr, target: Target, content: Inlines) -> Inline {
    // the traversal here is slightly inefficient because we need
    // to non-destructively check for the goodness of the content
    // before deciding to destructively create a Cite

    let is_semicolon = |inline: &Inline| match &inline {
        Inline::Str(Str { text }) => text == ";",
        _ => false,
    };

    let is_good_cite = content.split(is_semicolon).all(|slice| {
        slice.iter().any(|inline| match inline {
            Inline::Cite(_) => true,
            _ => false,
        })
    });

    if !is_good_cite {
        // if the content is not a good Cite, we backtrack and return a Span
        return make_span_inline(attr, target, content);
    }

    // we can now destructively create a Cite inline
    // from the content.

    // first we split the content along semicolons
    let citations: Vec<Citation> = content
        .split(is_semicolon)
        .map(|slice| {
            let inlines = slice.to_vec();
            let mut cite: Option<Cite> = None;
            let mut prefix: Inlines = vec![];
            let mut suffix: Inlines = vec![];

            // now we build prefix and suffix around a Cite. If there's none, we return None
            inlines.into_iter().for_each(|inline| {
                if cite == None {
                    if let Inline::Cite(c) = inline {
                        cite = Some(c);
                    } else {
                        prefix.push(inline);
                    }
                } else {
                    suffix.push(inline);
                }
            });
            let Some(mut c) = cite else {
                panic!("Cite inline should have at least one citation, found none")
            };
            if c.citations.len() != 1 {
                panic!(
                    "Cite inline should have exactly one citation, found: {:?}",
                    c.citations
                );
            }
            let mut citation = c.citations.pop().unwrap();
            if citation.mode == CitationMode::AuthorInText {
                // if the mode is AuthorInText, it becomes NormalCitation inside
                // a compound cite
                citation.mode = CitationMode::NormalCitation;
            }
            citation.prefix = prefix;
            citation.suffix = suffix;
            citation
        })
        .collect();
    return Inline::Cite(Cite {
        citations,
        content: vec![],
    });
}

fn make_inline_leftover(node: &tree_sitter::Node, input_bytes: &[u8]) -> Inline {
    let text = node.utf8_text(input_bytes).unwrap().to_string();
    Inline::RawInline(RawInline {
        format: "quarto-internal-leftover".to_string(),
        text,
    })
}

fn make_block_leftover(node: &tree_sitter::Node, input_bytes: &[u8]) -> Block {
    let text = node.utf8_text(input_bytes).unwrap().to_string();
    Block::RawBlock(RawBlock {
        format: "quarto-internal-leftover".to_string(),
        text,
        filename: None,
        range: node_location(node),
    })
}

fn native_visitor<T: Write>(
    buf: &mut T,
    node: &tree_sitter::Node,
    children: Vec<(String, PandocNativeIntermediate)>,
    input_bytes: &[u8],
) -> PandocNativeIntermediate {
    let whitespace_re = Regex::new(r"\s+").unwrap();
    let indent_re = Regex::new(r"[ \t]+").unwrap();
    let escaped_double_quote_re = Regex::new("[\\\\][\"]").unwrap();
    let escaped_single_quote_re = Regex::new("[\\\\][']").unwrap();

    let node_text = || node.utf8_text(input_bytes).unwrap().to_string();

    let string_as_base_text = || {
        let location = node_location(node);
        let value = node_text();
        if value.starts_with('"') && value.ends_with('"') {
            let value = value[1..value.len() - 1].to_string();
            PandocNativeIntermediate::IntermediateBaseText(
                escaped_double_quote_re
                    .replace_all(&value, "\"")
                    .to_string(),
                location,
            )
        } else if value.starts_with('\'') && value.ends_with('\'') {
            let value = value[1..value.len() - 1].to_string();
            PandocNativeIntermediate::IntermediateBaseText(
                escaped_single_quote_re.replace_all(&value, "'").to_string(),
                location,
            )
        } else {
            // If not quoted, return as is
            PandocNativeIntermediate::IntermediateBaseText(value, location)
        }
    };
    let native_inline = |(node, child)| match child {
        PandocNativeIntermediate::IntermediateInline(inline) => inline,
        PandocNativeIntermediate::IntermediateBaseText(text, range) => {
            if let Some(_) = whitespace_re.find(&text) {
                Inline::Space(Space {
                    filename: None,
                    range,
                })
            } else {
                Inline::Str(Str { text })
            }
        }
        // as a special inline, we need to allow commonmark attributes
        // to show up in the document, so we can appropriately attach attributes
        // to headings and tables (through their captions) as needed
        //
        // see tests/cursed/002.qmd for why this cannot be parsed directly in
        // the block grammar.
        PandocNativeIntermediate::IntermediateAttr(attr) => Inline::Attr(attr),
        PandocNativeIntermediate::IntermediateUnknown(range) => {
            writeln!(
                buf,
                "Ignoring unexpected unknown node in native inline at ({}:{}): {:?}.",
                range.start.row + 1,
                range.start.column + 1,
                node
            )
            .unwrap();
            Inline::RawInline(RawInline {
                format: "quarto-internal-leftover".to_string(),
                text: node_text(),
            })
        }
        other => {
            writeln!(
                buf,
                "Ignoring unexpected unknown node in native_inline {:?}.",
                other
            )
            .unwrap();
            Inline::RawInline(RawInline {
                format: "quarto-internal-leftover".to_string(),
                text: node_text(),
            })
        }
    };
    let native_inlines = |children| {
        let mut inlines: Vec<Inline> = Vec::new();
        for (_, child) in children {
            match child {
                PandocNativeIntermediate::IntermediateInline(inline) => inlines.push(inline),
                PandocNativeIntermediate::IntermediateInlines(inner_inlines) => {
                    inlines.extend(inner_inlines)
                }
                PandocNativeIntermediate::IntermediateBaseText(text, range) => {
                    if let Some(_) = whitespace_re.find(&text) {
                        inlines.push(Inline::Space(Space {
                            filename: None,
                            range,
                        }))
                    } else {
                        inlines.push(Inline::Str(Str { text }))
                    }
                }
                _ => panic!("Unexpected child in link_text: {:?}", child),
            }
        }
        inlines
    };

    match node.kind() {
        "numeric_character_reference" => {
            // Convert numeric character references to their corresponding characters
            // &#x0040; => @, &#64; => @, etc
            let text = node_text();
            let char_value = if text.starts_with("&#x") || text.starts_with("&#X") {
                // Hexadecimal reference
                let hex_str = &text[3..text.len() - 1];
                u32::from_str_radix(hex_str, 16).ok()
            } else if text.starts_with("&#") {
                // Decimal reference
                let dec_str = &text[2..text.len() - 1];
                dec_str.parse::<u32>().ok()
            } else {
                None
            };

            let result_text = match char_value.and_then(char::from_u32) {
                Some(ch) => ch.to_string(),
                None => text, // If we can't parse it, return the original text
            };

            PandocNativeIntermediate::IntermediateBaseText(result_text, node_location(node))
        }

        "language"
        | "note_reference_id"
        | "citation_id_suppress_author"
        | "citation_id_author_in_text"
        | "link_destination"
        | "key_value_key"
        | "code_content"
        | "latex_content"
        | "text_base" => {
            PandocNativeIntermediate::IntermediateBaseText(node_text(), node_location(node))
        }
        "document" => {
            let mut blocks: Vec<Block> = Vec::new();
            children.into_iter().for_each(|(_, child)| {
                match child {
                    PandocNativeIntermediate::IntermediateBlock(block) => blocks.push(block),
                    PandocNativeIntermediate::IntermediateSection(section) => {
                        blocks.extend(section);
                    }
                    PandocNativeIntermediate::IntermediateMetadataString(text, range) => {
                        // for now we assume it's metadata and emit it as a rawblock
                        blocks.push(Block::RawBlock(RawBlock {
                            format: "quarto_minus_metadata".to_string(),
                            text,
                            filename: None,
                            range: range,
                        }));
                    }
                    _ => panic!("Expected Block or Section, got {:?}", child),
                }
            });
            PandocNativeIntermediate::IntermediatePandoc(Pandoc { blocks })
        }
        "section" => {
            let mut blocks: Vec<Block> = Vec::new();
            children.into_iter().for_each(|(node, child)| {
                if node == "block_continuation" {
                    return;
                }
                match child {
                    PandocNativeIntermediate::IntermediateBlock(block) => blocks.push(block),
                    PandocNativeIntermediate::IntermediateSection(section) => {
                        blocks.extend(section);
                    }
                    PandocNativeIntermediate::IntermediateMetadataString(text, range) => {
                        // for now we assume it's metadata and emit it as a rawblock
                        blocks.push(Block::RawBlock(RawBlock {
                            format: "quarto_minus_metadata".to_string(),
                            text,
                            filename: None,
                            range: range,
                        }));
                    }
                    _ => panic!("Expected Block or Section, got {:?} {:?}", node, child),
                }
            });
            PandocNativeIntermediate::IntermediateSection(blocks)
        }
        "paragraph" => {
            let mut inlines: Vec<Inline> = Vec::new();
            for (node, child) in children {
                if node == "block_continuation" {
                    continue; // skip block continuation nodes
                }
                if let PandocNativeIntermediate::IntermediateInline(inline) = child {
                    inlines.push(inline);
                } else if let PandocNativeIntermediate::IntermediateInlines(inner_inlines) = child {
                    inlines.extend(inner_inlines);
                }
            }
            PandocNativeIntermediate::IntermediateBlock(Block::Paragraph(Paragraph {
                content: inlines,

                filename: None,
                range: node_location(node),
            }))
        }
        "indented_code_block" => {
            let mut content: String = String::new();
            let outer_range = node_location(node);
            // first, find the beginning of the contents in the node itself
            let outer_string = node_text();
            let mut start_offset = indent_re.find(&outer_string).map_or(0, |m| m.end());

            for (node, children) in children {
                if node == "block_continuation" {
                    // append all content up to the beginning of this continuation
                    match children {
                        PandocNativeIntermediate::IntermediateUnknown(range) => {
                            // Calculate the relative offset of the continuation within outer_string
                            let continuation_start =
                                range.start.offset.saturating_sub(outer_range.start.offset);
                            let continuation_end =
                                range.end.offset.saturating_sub(outer_range.start.offset);

                            // Append content before this continuation
                            if continuation_start > start_offset
                                && continuation_start <= outer_string.len()
                            {
                                content.push_str(&outer_string[start_offset..continuation_start]);
                            }

                            // Update start_offset to after this continuation
                            start_offset = continuation_end.min(outer_string.len());
                        }
                        _ => panic!("Unexpected {:?} inside indented_code_block", children),
                    }
                }
            }
            // append the remaining content after the last continuation
            content.push_str(&outer_string[start_offset..]);
            // TODO this will require careful encoding of the source map when we get to that point
            return PandocNativeIntermediate::IntermediateBlock(Block::CodeBlock(CodeBlock {
                attr: empty_attr(),
                text: content.trim_end().to_string(),
                filename: None,
                range: outer_range,
            }));
        }
        "fenced_code_block" => {
            let mut content: String = String::new();
            let mut attr: Attr = empty_attr();
            let mut raw_format: Option<String> = None;
            for (node, child) in children {
                if node == "block_continuation" {
                    continue; // skip block continuation nodes
                }
                if node == "code_fence_content" {
                    let PandocNativeIntermediate::IntermediateBaseText(text, _) = child else {
                        panic!("Expected BaseText in code_fence_content, got {:?}", child)
                    };
                    content = text;
                } else if node == "commonmark_attribute" {
                    let PandocNativeIntermediate::IntermediateAttr(a) = child else {
                        panic!("Expected Attr in commonmark_attribute, got {:?}", child)
                    };
                    attr = a;
                } else if node == "raw_attribute" {
                    let PandocNativeIntermediate::IntermediateRawFormat(format, _) = child else {
                        panic!("Expected RawFormat in raw_attribute, got {:?}", child)
                    };
                    raw_format = Some(format);
                } else if node == "language_attribute" {
                    let PandocNativeIntermediate::IntermediateBaseText(lang, _) = child else {
                        panic!("Expected BaseText in language_attribute, got {:?}", child)
                    };
                    attr.1.push(lang); // set the language
                } else if node == "info_string" {
                    let PandocNativeIntermediate::IntermediateAttr(inner_attr) = child else {
                        panic!("Expected Attr in info_string, got {:?}", child)
                    };
                    attr = inner_attr;
                }
            }
            let location = node_location(node);

            // it might be the case (because of tree-sitter error recovery)
            // that the content does not end with a newline, so we ensure it does before popping
            if content.ends_with('\n') {
                content.pop(); // remove the trailing newline
            }

            if let Some(format) = raw_format {
                return PandocNativeIntermediate::IntermediateBlock(Block::RawBlock(RawBlock {
                    format,
                    text: content,
                    filename: None,
                    range: location,
                }));
            }
            return PandocNativeIntermediate::IntermediateBlock(Block::CodeBlock(CodeBlock {
                attr,
                text: content,
                filename: None,
                range: location,
            }));
        }
        "attribute" => {
            for (node, child) in children {
                match child {
                    PandocNativeIntermediate::IntermediateAttr(attr) => {
                        if node == "commonmark_attribute" {
                            return PandocNativeIntermediate::IntermediateAttr(attr);
                        } else if node == "raw_attribute" {
                            panic!("Unexpected raw attribute in attribute: {:?}", attr);
                        } else {
                            panic!("Unexpected attribute node: {}", node);
                        }
                    }
                    _ => panic!("Unexpected child in attribute: {:?}", child),
                }
            }
            panic!("No commonmark_attribute found in attribute node");
        }
        "commonmark_attribute" => {
            let mut attr = ("".to_string(), vec![], HashMap::new());
            children.into_iter().for_each(|(node, child)| match child {
                PandocNativeIntermediate::IntermediateBaseText(id, _) => {
                    if node == "id_specifier" {
                        attr.0 = id;
                    } else if node == "class_specifier" {
                        attr.1.push(id);
                    } else {
                        panic!("Unexpected commonmark_attribute node: {}", node);
                    }
                }
                PandocNativeIntermediate::IntermediateKeyValueSpec(spec) => {
                    for (key, value) in spec {
                        attr.2.insert(key, value);
                    }
                }
                PandocNativeIntermediate::IntermediateUnknown(_) => {}
                _ => panic!("Unexpected child in commonmark_attribute: {:?}", child),
            });
            PandocNativeIntermediate::IntermediateAttr(attr)
        }
        "class_specifier" => {
            let id = node_text().split_off(1);
            PandocNativeIntermediate::IntermediateBaseText(id, node_location(node))
        }
        "id_specifier" => {
            let id = node_text().split_off(1);
            PandocNativeIntermediate::IntermediateBaseText(id, node_location(node))
        }
        "shortcode_naked_string" | "shortcode_name" => {
            let id = node_text().to_string();
            PandocNativeIntermediate::IntermediateShortcodeArg(
                ShortcodeArg::String(id),
                node_location(node),
            )
        }
        "shortcode_string" => {
            let PandocNativeIntermediate::IntermediateBaseText(id, _) = string_as_base_text()
            else {
                panic!(
                    "Expected BaseText in shortcode_string, got {:?}",
                    string_as_base_text()
                )
            };
            PandocNativeIntermediate::IntermediateShortcodeArg(
                ShortcodeArg::String(id),
                node_location(node),
            )
        }
        "key_value_value" => string_as_base_text(),
        "link_title" => {
            let title = node_text();
            let title = title[1..title.len() - 1].to_string();
            PandocNativeIntermediate::IntermediateBaseText(title, node_location(node))
        }
        "link_text" => PandocNativeIntermediate::IntermediateInlines(native_inlines(children)),
        "image" => {
            let mut attr = ("".to_string(), vec![], HashMap::new());
            let mut target: Target = ("".to_string(), "".to_string());
            let mut content: Vec<Inline> = Vec::new();
            for (node, child) in children {
                if node == "image_description" {
                    let PandocNativeIntermediate::IntermediateInlines(inlines) = child else {
                        panic!("Expected inlines in image_description, got {:?}", child)
                    };
                    content.extend(inlines);
                    continue;
                }
                match child {
                    PandocNativeIntermediate::IntermediateAttr(a) => attr = a,
                    PandocNativeIntermediate::IntermediateBaseText(text, _) => {
                        if node == "link_destination" {
                            target.0 = text; // URL
                        } else if node == "link_title" {
                            target.1 = text; // Title
                        } else {
                            panic!("Unexpected image node: {}", node);
                        }
                    }
                    PandocNativeIntermediate::IntermediateUnknown(_) => {}
                    PandocNativeIntermediate::IntermediateInlines(inlines) => {
                        content.extend(inlines)
                    }
                    PandocNativeIntermediate::IntermediateInline(inline) => content.push(inline),
                    _ => panic!("Unexpected child in inline_link: {:?}", child),
                }
            }
            PandocNativeIntermediate::IntermediateInline(Inline::Image(Image {
                attr,
                content,
                target,
            }))
        }
        "image_description" => {
            PandocNativeIntermediate::IntermediateInlines(native_inlines(children))
        }
        "inline_link" => {
            let mut attr = ("".to_string(), vec![], HashMap::new());
            let mut target = ("".to_string(), "".to_string());
            let mut content: Vec<Inline> = Vec::new();

            for (node, child) in children {
                match child {
                    PandocNativeIntermediate::IntermediateAttr(a) => attr = a,
                    PandocNativeIntermediate::IntermediateBaseText(text, _) => {
                        if node == "link_destination" {
                            target.0 = text; // URL
                        } else if node == "link_title" {
                            target.1 = text; // Title
                        } else {
                            panic!("Unexpected inline_link node: {}", node);
                        }
                    }
                    PandocNativeIntermediate::IntermediateUnknown(_) => {}
                    PandocNativeIntermediate::IntermediateInlines(inlines) => {
                        content.extend(inlines)
                    }
                    PandocNativeIntermediate::IntermediateInline(inline) => content.push(inline),
                    _ => panic!("Unexpected child in inline_link: {:?}", child),
                }
            }
            let has_citations = content
                .iter()
                .any(|inline| matches!(inline, Inline::Cite(_)));

            // an inline link might be a Cite if it has citations, no destination, and no title
            // and no attributes
            let is_cite = has_citations && is_empty_target(&target) && is_empty_attr(&attr);

            return PandocNativeIntermediate::IntermediateInline(if is_cite {
                make_cite_inline(attr, target, content)
            } else {
                make_span_inline(attr, target, content)
            });
        }
        "key_value_specifier" => {
            let mut spec = HashMap::new();
            let mut current_key: Option<String> = None;
            for (node, child) in children {
                if let PandocNativeIntermediate::IntermediateBaseText(value, _) = child {
                    if node == "key_value_key" {
                        current_key = Some(value);
                    } else if node == "key_value_value" {
                        if let Some(key) = current_key.take() {
                            spec.insert(key, value);
                        } else {
                            panic!("Found key_value_value without a preceding key_value_key");
                        }
                    } else {
                        writeln!(buf, "Unexpected key_value_specifier node: {}", node).unwrap();
                    }
                }
            }
            PandocNativeIntermediate::IntermediateKeyValueSpec(spec)
        }
        "raw_specifier" => {
            // like code_content but skipping first character
            let raw = node_text();
            if raw.chars().nth(0) == Some('<') {
                PandocNativeIntermediate::IntermediateBaseText(
                    "pandoc-reader:".to_string() + &raw[1..],
                    node_location(node),
                )
            } else {
                PandocNativeIntermediate::IntermediateBaseText(
                    raw[1..].to_string(),
                    node_location(node),
                )
            }
        }
        "emphasis" => {
            let inlines: Vec<Inline> = children
                .into_iter()
                .filter(|(node, _)| {
                    node != "emphasis_delimiter" // skip emphasis delimiters
                })
                .map(native_inline)
                .collect();
            PandocNativeIntermediate::IntermediateInline(Inline::Emph(Emph { content: inlines }))
        }
        "strong_emphasis" => {
            let inlines: Vec<Inline> = children
                .into_iter()
                .filter(|(node, _)| {
                    node != "emphasis_delimiter" // skip emphasis delimiters
                })
                .map(native_inline)
                .collect();
            PandocNativeIntermediate::IntermediateInline(Inline::Strong(Strong {
                content: inlines,
            }))
        }
        "inline" => {
            let inlines: Vec<Inline> = children.into_iter().map(native_inline).collect();
            PandocNativeIntermediate::IntermediateInlines(inlines)
        }
        "citation" => {
            let mut citation_type = CitationMode::NormalCitation;
            let mut citation_id = String::new();
            for (node, child) in children {
                if node == "citation_id_suppress_author" {
                    citation_type = CitationMode::SuppressAuthor;
                    if let PandocNativeIntermediate::IntermediateBaseText(id, _) = child {
                        citation_id = id;
                    } else {
                        panic!(
                            "Expected BaseText in citation_id_suppress_author, got {:?}",
                            child
                        );
                    }
                } else if node == "citation_id_author_in_text" {
                    citation_type = CitationMode::AuthorInText;
                    if let PandocNativeIntermediate::IntermediateBaseText(id, _) = child {
                        citation_id = id;
                    } else {
                        panic!(
                            "Expected BaseText in citation_id_author_in_text, got {:?}",
                            child
                        );
                    }
                }
            }
            PandocNativeIntermediate::IntermediateInline(Inline::Cite(Cite {
                citations: vec![Citation {
                    id: citation_id,
                    prefix: vec![],
                    suffix: vec![],
                    mode: citation_type,
                    note_num: 0, // this needs to be set later
                    hash: 0,
                }],
                content: vec![Inline::Str(Str { text: node_text() })],
            }))
        }
        "note_reference" => {
            let mut id = String::new();
            for (node, child) in children {
                if node == "note_reference_delimiter" {
                    // This is a marker node, we don't need to do anything with it
                } else if node == "note_reference_id" {
                    if let PandocNativeIntermediate::IntermediateBaseText(text, _) = child {
                        id = text;
                    } else {
                        panic!("Expected BaseText in note_reference_id, got {:?}", child);
                    }
                } else {
                    panic!("Unexpected note_reference node: {}", node);
                }
            }
            PandocNativeIntermediate::IntermediateInline(Inline::NoteReference(NoteReference {
                id,
                range: node_location(node),
            }))
        }
        "shortcode" | "shortcode_escaped" => {
            let is_escaped = node.kind() == "shortcode_escaped";
            let mut name = String::new();
            let mut positional_args: Vec<ShortcodeArg> = Vec::new();
            let mut keyword_args: HashMap<String, ShortcodeArg> = HashMap::new();
            for (node, child) in children {
                match (node.as_str(), child) {
                    (
                        "shortcode_naked_string",
                        PandocNativeIntermediate::IntermediateShortcodeArg(
                            ShortcodeArg::String(text),
                            _,
                        ),
                    )
                    | (
                        "shortcode_name",
                        PandocNativeIntermediate::IntermediateShortcodeArg(
                            ShortcodeArg::String(text),
                            _,
                        ),
                    )
                    | (
                        "shortcode_string",
                        PandocNativeIntermediate::IntermediateShortcodeArg(
                            ShortcodeArg::String(text),
                            _,
                        ),
                    ) => {
                        if name.is_empty() {
                            name = text;
                        } else {
                            positional_args.push(ShortcodeArg::String(text));
                        }
                    }
                    (
                        "shortcode_keyword_param",
                        PandocNativeIntermediate::IntermediateShortcodeArg(
                            ShortcodeArg::KeyValue(spec),
                            _,
                        ),
                    ) => {
                        for (key, value) in spec {
                            keyword_args.insert(key, value);
                        }
                    }
                    (
                        "shortcode",
                        PandocNativeIntermediate::IntermediateInline(Inline::Shortcode(arg)),
                    ) => {
                        positional_args.push(ShortcodeArg::Shortcode(arg));
                    }
                    (
                        "shortcode_number",
                        PandocNativeIntermediate::IntermediateShortcodeArg(arg, _),
                    )
                    | (
                        "shortcode_boolean",
                        PandocNativeIntermediate::IntermediateShortcodeArg(arg, _),
                    ) => {
                        positional_args.push(arg);
                    }
                    ("shortcode_delimiter", _) => {
                        // This is a marker node, we don't need to do anything with it
                    }
                    (child_type, child) => panic!(
                        "Unexpected node in {:?}: {:?} {:?}",
                        node,
                        child_type,
                        child.clone()
                    ),
                }
            }
            PandocNativeIntermediate::IntermediateInline(Inline::Shortcode(Shortcode {
                is_escaped,
                name,
                positional_args,
                keyword_args,
            }))
        }
        "shortcode_keyword_param" => {
            let mut result = HashMap::new();
            let mut name = String::new();
            for (node, child) in children {
                match node.as_str() {
                    "shortcode_name" => {
                        let PandocNativeIntermediate::IntermediateShortcodeArg(
                            ShortcodeArg::String(text),
                            _,
                        ) = child
                        else {
                            panic!("Expected BaseText in shortcode_name, got {:?}", child)
                        };
                        if name.is_empty() {
                            name = text;
                        } else {
                            result.insert(name.clone(), ShortcodeArg::String(text));
                        }
                    }
                    "shortcode_string"
                    | "shortcode_number"
                    | "shortcode_naked_string"
                    | "shortcode_boolean" => {
                        let PandocNativeIntermediate::IntermediateShortcodeArg(arg, _) = child
                        else {
                            panic!("Expected ShortcodeArg in shortcode_string, got {:?}", child)
                        };
                        result.insert(name.clone(), arg);
                    }
                    "block_continuation" => {
                        // This is a marker node, we don't need to do anything with it
                    }
                    _ => {
                        writeln!(buf, "Warning: Unhandled node kind: {}", node).unwrap();
                    }
                }
            }
            let range = node_location(node);
            PandocNativeIntermediate::IntermediateShortcodeArg(
                ShortcodeArg::KeyValue(result),
                range,
            )
        }
        "shortcode_boolean" => {
            let value = node_text();
            let value = match value.as_str() {
                "true" => ShortcodeArg::Boolean(true),
                "false" => ShortcodeArg::Boolean(false),
                _ => panic!("Unexpected shortcode_boolean value: {}", value),
            };
            let range = node_location(node);
            PandocNativeIntermediate::IntermediateShortcodeArg(value, range)
        }
        "shortcode_number" => {
            let value = node_text();
            let range = node_location(node);
            let Ok(num) = value.parse::<f64>() else {
                panic!("Invalid shortcode_number: {}", value)
            };
            PandocNativeIntermediate::IntermediateShortcodeArg(ShortcodeArg::Number(num), range)
        }
        "code_fence_content" => {
            let start = node.range().start_byte;
            let end = node.range().end_byte;

            // This is a code block, we need to extract the content
            // by removing block_continuation markers
            let mut current_location = start;

            let mut content = String::new();
            for (child_node, child) in children {
                if child_node == "block_continuation" {
                    let PandocNativeIntermediate::IntermediateUnknown(child_range) = child else {
                        panic!(
                            "Expected IntermediateUnknown in block_continuation, got {:?}",
                            child
                        )
                    };
                    let slice_before_continuation =
                        &input_bytes[current_location..child_range.start.offset];
                    content.push_str(std::str::from_utf8(slice_before_continuation).unwrap());
                    current_location = child_range.end.offset;
                }
            }
            // Add the remaining content after the last block_continuation
            if current_location < end {
                let slice_after_continuation = &input_bytes[current_location..end];
                content.push_str(std::str::from_utf8(slice_after_continuation).unwrap());
            }
            PandocNativeIntermediate::IntermediateBaseText(content, node_location(node))
        }
        "list_marker_parenthesis" | "list_marker_dot" => {
            // we need to extract the marker number
            let marker_text = node
                .utf8_text(input_bytes)
                .unwrap()
                .trim_end()
                .trim_end_matches('.')
                .trim_end_matches(')')
                .to_string();
            let marker_number: usize = marker_text
                .parse()
                .unwrap_or_else(|_| panic!("Invalid list marker number: {}", marker_text));
            PandocNativeIntermediate::IntermediateOrderedListMarker(
                marker_number,
                node_location(node),
            )
        }
        // These are marker nodes, we don't need to do anything with it
        "block_quote_marker"
        | "list_marker_minus"
        | "list_marker_star"
        | "list_marker_plus"
        | "block_continuation"
        | "fenced_code_block_delimiter"
        | "note_reference_delimiter"
        | "shortcode_delimiter"
        | "citation_delimiter"
        | "code_span_delimiter"
        | "single_quoted_span_delimiter"
        | "double_quoted_span_delimiter"
        | "superscript_delimiter"
        | "subscript_delimiter"
        | "strikeout_delimiter"
        | "emphasis_delimiter" => {
            PandocNativeIntermediate::IntermediateUnknown(node_location(node))
        }
        "soft_line_break" => {
            PandocNativeIntermediate::IntermediateInline(Inline::SoftBreak(SoftBreak {
                filename: None,
                range: node_location(node),
            }))
        }
        "hard_line_break" => {
            PandocNativeIntermediate::IntermediateInline(Inline::LineBreak(LineBreak {
                filename: None,
                range: node_location(node),
            }))
        }
        "latex_span_delimiter" => {
            let str = node.utf8_text(input_bytes).unwrap();
            let range = node_location(node);
            if str == "$" {
                PandocNativeIntermediate::IntermediateLatexInlineDelimiter(range)
            } else if str == "$$" {
                PandocNativeIntermediate::IntermediateLatexDisplayDelimiter(range)
            } else {
                writeln!(
                    buf,
                    "Warning: Unrecognized latex_span_delimiter: {} Will assume inline delimiter",
                    str
                )
                .unwrap();
                PandocNativeIntermediate::IntermediateLatexInlineDelimiter(range)
            }
        }
        "inline_note" => {
            let inlines: Vec<_> = children
                .into_iter()
                .filter(|(node, _)| node != "inline_note_delimiter")
                .map(native_inline)
                .collect();
            PandocNativeIntermediate::IntermediateInline(Inline::Note(Note {
                content: vec![Block::Paragraph(Paragraph {
                    content: inlines,

                    filename: None,
                    range: node_location(node),
                })],
            }))
        }
        "superscript" => {
            let inlines: Vec<_> = children
                .into_iter()
                .filter(|(node, _)| node != "superscript_delimiter")
                .map(native_inline)
                .collect();
            PandocNativeIntermediate::IntermediateInline(Inline::Superscript(Superscript {
                content: inlines,
            }))
        }
        "subscript" => {
            let inlines: Vec<_> = children
                .into_iter()
                .filter(|(node, _)| node != "subscript_delimiter")
                .map(native_inline)
                .collect();
            PandocNativeIntermediate::IntermediateInline(Inline::Subscript(Subscript {
                content: inlines,
            }))
        }
        "strikeout" => {
            let inlines: Vec<_> = children
                .into_iter()
                .filter(|(node, _)| node != "strikeout_delimiter")
                .map(native_inline)
                .collect();
            PandocNativeIntermediate::IntermediateInline(Inline::Strikeout(Strikeout {
                content: inlines,
            }))
        }

        "quoted_span" => {
            let mut quote_type = QuoteType::SingleQuote;
            let inlines: Vec<_> = children
                .into_iter()
                .filter(|(node, intermediate)| {
                    if node == "single_quoted_span_delimiter" {
                        quote_type = QuoteType::SingleQuote;
                        false // skip the opening delimiter
                    } else if node == "double_quoted_span_delimiter" {
                        quote_type = QuoteType::DoubleQuote;
                        false // skip the opening delimiter
                    } else {
                        match intermediate {
                            PandocNativeIntermediate::IntermediateInline(_) => true,
                            PandocNativeIntermediate::IntermediateBaseText(_, _) => true,
                            _ => false,
                        }
                    }
                })
                .map(native_inline)
                .collect();
            PandocNativeIntermediate::IntermediateInline(Inline::Quoted(Quoted {
                quote_type,
                content: inlines,
            }))
        }
        "code_span" => {
            let mut is_raw: Option<String> = None;
            let mut attr = ("".to_string(), vec![], HashMap::new());
            let mut inlines: Vec<_> = children
                .into_iter()
                .map(|(node_name, child)| {
                    let range = node_location(node);
                    match child {
                        PandocNativeIntermediate::IntermediateAttr(a) => {
                            attr = a;
                            // IntermediateUnknown here "consumes" the node
                            (
                                node_name,
                                PandocNativeIntermediate::IntermediateUnknown(range),
                            )
                        }
                        PandocNativeIntermediate::IntermediateRawFormat(raw, _) => {
                            is_raw = Some(raw);
                            // IntermediateUnknown here "consumes" the node
                            (
                                node_name,
                                PandocNativeIntermediate::IntermediateUnknown(range),
                            )
                        }
                        _ => (node_name, child),
                    }
                })
                .filter(|(_, child)| {
                    match child {
                        PandocNativeIntermediate::IntermediateUnknown(_) => false, // skip unknown nodes
                        _ => true, // keep other nodes
                    }
                })
                .collect();
            assert!(
                inlines.len() == 1,
                "Expected exactly one inline in code_span, got {}",
                inlines.len()
            );
            let (_, child) = inlines.remove(0);
            let PandocNativeIntermediate::IntermediateBaseText(text, _) = child else {
                panic!("Expected BaseText in code_span, got {:?}", child);
            };
            if let Some(raw) = is_raw {
                PandocNativeIntermediate::IntermediateInline(Inline::RawInline(RawInline {
                    format: raw,
                    text,
                }))
            } else {
                PandocNativeIntermediate::IntermediateInline(Inline::Code(Code { attr, text }))
            }
        }
        "latex_span" => {
            let mut is_inline_math = false;
            let mut is_display_math = false;
            let mut inlines: Vec<_> = children
                .into_iter()
                .filter(|(_, child)| {
                    if matches!(
                        child,
                        PandocNativeIntermediate::IntermediateLatexInlineDelimiter(_)
                    ) {
                        is_inline_math = true;
                        false // skip the delimiter
                    } else if matches!(
                        child,
                        PandocNativeIntermediate::IntermediateLatexDisplayDelimiter(_)
                    ) {
                        is_display_math = true;
                        false // skip the delimiter
                    } else {
                        true // keep other nodes
                    }
                })
                .collect();
            assert!(
                inlines.len() == 1,
                "Expected exactly one inline in latex_span, got {}",
                inlines.len()
            );
            if is_inline_math && is_display_math {
                panic!("Unexpected both inline and display math in latex_span");
            }
            if !is_inline_math && !is_display_math {
                panic!("Expected either inline or display math in latex_span, got neither");
            }
            let math_type = if is_inline_math {
                MathType::InlineMath
            } else {
                MathType::DisplayMath
            };
            let (_, child) = inlines.remove(0);
            let PandocNativeIntermediate::IntermediateBaseText(text, _) = child else {
                panic!("Expected BaseText in latex_span, got {:?}", child)
            };
            PandocNativeIntermediate::IntermediateInline(Inline::Math(Math {
                math_type: math_type,
                text,
            }))
        }
        "list" => {
            // a list is loose if it has at least one loose item
            // an item is loose if
            //   - it has more than one paragraph in the list
            //   - it is a single paragraph with space between it and the next
            //     beginning of list item. There must be a next item for this to be true
            //     but the next item might not itself be a paragraph.

            let mut has_loose_item = false;
            let mut last_para_range: Option<Range> = None;
            let mut list_items: Vec<Blocks> = Vec::new();
            let mut is_ordered_list: Option<ListAttributes> = None;

            for (node, child) in children {
                if node == "block_continuation" {
                    // this is a marker node, we don't need to do anything with it
                    continue;
                }
                if node == "list_marker_parenthesis" || node == "list_marker_dot" {
                    // this is an ordered list, so we need to set the flag
                    let PandocNativeIntermediate::IntermediateOrderedListMarker(marker_number, _) =
                        child
                    else {
                        panic!("Expected OrderedListMarker in list, got {:?}", child);
                    };

                    is_ordered_list = Some((
                        marker_number,
                        ListNumberStyle::Decimal,
                        match node.as_str() {
                            "list_marker_parenthesis" => ListNumberDelim::OneParen,
                            "list_marker_dot" => ListNumberDelim::Period,
                            _ => panic!("Unexpected list marker node: {}", node),
                        },
                    ));
                }

                if node != "list_item" {
                    panic!("Expected list_item in list, got {}", node);
                }
                let PandocNativeIntermediate::IntermediateListItem(
                    blocks,
                    child_range,
                    ordered_list,
                ) = child
                else {
                    panic!("Expected Blocks in list_item, got {:?}", child);
                };
                if is_ordered_list == None {
                    match ordered_list {
                        attr @ Some(_) => is_ordered_list = attr,
                        _ => {}
                    }
                }

                // is the last item loose? Check the last paragraph range
                if let Some(ref last_range) = last_para_range {
                    if last_range.end.row != child_range.start.row {
                        // if the last paragraph ends on a different line than the current item starts,
                        // then the last item was loose, mark it
                        has_loose_item = true;
                    }
                }

                // is this item definitely loose?
                if blocks
                    .iter()
                    .filter(|block| {
                        if let Block::Paragraph(_) = block {
                            true
                        } else {
                            false
                        }
                    })
                    .count()
                    > 1
                {
                    has_loose_item = true;

                    // technically, we don't need to worry about
                    // last paragraph range after setting has_loose_item,
                    // but we do it in case we want to use it later
                    last_para_range = None;
                    list_items.push(blocks);
                    continue;
                }

                // is this item possibly loose?
                if blocks.len() == 1 {
                    if let Some(Block::Paragraph(para)) = blocks.first() {
                        // yes, so store the range and wait to finish the check on
                        // next item
                        last_para_range = Some(para.range.clone());
                    } else {
                        // if the first block is not a paragraph, it's not loose
                        last_para_range = None;
                    }
                }
                list_items.push(blocks);
            }

            let content = if has_loose_item {
                // the AST representation of a loose bullet list is
                // the same as what we've been building, so just return it
                list_items
            } else {
                // turn list into tight list by replacing eligible Paragraph nodes
                // Plain nodes.
                list_items
                    .into_iter()
                    .map(|mut blocks| {
                        if blocks.len() != 1 {
                            return blocks;
                        }
                        let first = blocks.pop().unwrap();
                        let Block::Paragraph(Paragraph {
                            content,
                            filename,
                            range,
                        }) = first
                        else {
                            return vec![first];
                        };
                        vec![Block::Plain(Plain {
                            content: content,
                            filename: filename,
                            range: range,
                        })]
                    })
                    .collect()
            };

            match is_ordered_list {
                Some(attr) => {
                    PandocNativeIntermediate::IntermediateBlock(Block::OrderedList(OrderedList {
                        attr,
                        content,
                        filename: None,
                        range: node_location(node),
                    }))
                }
                None => {
                    PandocNativeIntermediate::IntermediateBlock(Block::BulletList(BulletList {
                        content,
                        filename: None,
                        range: node_location(node),
                    }))
                }
            }
        }
        "list_item" => {
            let mut list_attr: Option<ListAttributes> = None;
            let children = children
                .into_iter()
                .filter(|(node, child)| {
                    if node == "list_marker_dot" || node == "list_marker_parenthesis" {
                        // this is an ordered list, so we need to set the flag
                        let PandocNativeIntermediate::IntermediateOrderedListMarker(
                            marker_number,
                            _,
                        ) = child
                        else {
                            panic!("Expected OrderedListMarker in list_item, got {:?}", child);
                        };
                        list_attr = Some((
                            *marker_number,
                            ListNumberStyle::Decimal,
                            match node.as_str() {
                                "list_marker_parenthesis" => ListNumberDelim::OneParen,
                                "list_marker_dot" => ListNumberDelim::Period,
                                _ => panic!("Unexpected list marker node: {}", node),
                            },
                        ));
                        return false; // skip the marker node
                    }
                    matches!(child, PandocNativeIntermediate::IntermediateBlock(_))
                })
                .map(|(_, child)| {
                    let PandocNativeIntermediate::IntermediateBlock(block) = child else {
                        panic!("Expected Block in paragraph, got {:?}", child);
                    };
                    block
                })
                .collect();
            return PandocNativeIntermediate::IntermediateListItem(
                children,
                node_location(node),
                list_attr,
            );
        }
        "info_string" => {
            for (_, child) in children {
                match child {
                    PandocNativeIntermediate::IntermediateBaseText(text, _) => {
                        return PandocNativeIntermediate::IntermediateAttr((
                            "".to_string(),
                            vec![text],
                            HashMap::new(),
                        ));
                    }
                    _ => {}
                }
            }
            panic!("Expected info_string to have a string, but found none");
        }
        "language_attribute" => {
            for (_, child) in children {
                match child {
                    PandocNativeIntermediate::IntermediateBaseText(text, range) => {
                        return PandocNativeIntermediate::IntermediateBaseText(
                            "{".to_string() + &text + "}",
                            range,
                        );
                    }
                    _ => {}
                }
            }
            panic!("Expected language_attribute to have a language, but found none");
        }
        "raw_attribute" => {
            for (_, child) in children {
                let range = node_location(node);
                match child {
                    PandocNativeIntermediate::IntermediateBaseText(raw, _) => {
                        return PandocNativeIntermediate::IntermediateRawFormat(raw, range);
                    }
                    _ => {}
                }
            }
            panic!("Expected raw_attribute to have a format, but found none");
        }
        "block_quote" => {
            let mut content: Blocks = Vec::new();
            for (node_type, child) in children {
                if node_type == "block_quote_marker" {
                    if matches!(child, PandocNativeIntermediate::IntermediateUnknown(_)) {
                        if node_type != "block_continuation" {
                            writeln!(
                                buf,
                                "Warning: Unhandled node kind in block_quote: {}, {:?}",
                                node_type, child,
                            )
                            .unwrap();
                        }
                    }
                    continue;
                }
                match child {
                    PandocNativeIntermediate::IntermediateBlock(block) => {
                        content.push(block);
                    }
                    _ => {
                        writeln!(
                        buf,
                        "[block_quote] Will ignore unknown node. Expected Block in block_quote, got {:?}",
                        child
                        ).unwrap();
                    }
                }
            }
            PandocNativeIntermediate::IntermediateBlock(Block::BlockQuote(BlockQuote {
                content,
                filename: None,
                range: node_location(node),
            }))
        }
        "fenced_div_block" => {
            let mut attr: Attr = ("".to_string(), vec![], HashMap::new());
            let mut content: Vec<Block> = Vec::new();
            for (node, child) in children {
                if node == "block_continuation" {
                    continue;
                }
                match child {
                    PandocNativeIntermediate::IntermediateAttr(a) => {
                        attr = a;
                    }
                    PandocNativeIntermediate::IntermediateBlock(block) => {
                        content.push(block);
                    }
                    PandocNativeIntermediate::IntermediateSection(blocks) => {
                        content.extend(blocks);
                    }
                    _ => {
                        writeln!(
                            buf,
                            "Warning: Unhandled node kind in fenced_div_block: {:?} {:?}",
                            node, child
                        )
                        .unwrap();
                    }
                }
            }
            return PandocNativeIntermediate::IntermediateBlock(Block::Div(Div {
                attr,
                content,
                filename: None,
                range: node_location(node),
            }));
        }
        "atx_heading" => {
            let mut level = 0;
            let mut content: Vec<Inline> = Vec::new();
            let mut attr: Attr = ("".to_string(), vec![], HashMap::new());
            for (node, child) in children {
                if node == "block_continuation" {
                    continue;
                    // This is a marker node, we don't need to do anything with it
                } else if node == "atx_h1_marker" {
                    level = 1;
                } else if node == "atx_h2_marker" {
                    level = 2;
                } else if node == "atx_h3_marker" {
                    level = 3;
                } else if node == "atx_h4_marker" {
                    level = 4;
                } else if node == "atx_h5_marker" {
                    level = 5;
                } else if node == "atx_h6_marker" {
                    level = 6;
                } else if node == "inline" {
                    if let PandocNativeIntermediate::IntermediateInlines(inlines) = child {
                        content.extend(inlines);
                    } else {
                        panic!("Expected Inlines in atx_heading, got {:?}", child);
                    }
                } else if node == "attribute" {
                    if let PandocNativeIntermediate::IntermediateAttr(inner_attr) = child {
                        attr = inner_attr;
                    } else {
                        panic!("Expected Attr in attribute, got {:?}", child);
                    }
                } else {
                    writeln!(buf, "Warning: Unhandled node kind in atx_heading: {}", node).unwrap();
                }
            }
            PandocNativeIntermediate::IntermediateBlock(Block::Header(Header {
                level,
                attr,
                content,
                filename: None,
                range: node_location(node),
            }))
        }
        "thematic_break" => {
            PandocNativeIntermediate::IntermediateBlock(Block::HorizontalRule(HorizontalRule {
                filename: None,
                range: node_location(node),
            }))
        }
        "backslash_escape" => {
            // This is a backslash escape, we need to extract the content
            // by removing the backslash
            let text = node.utf8_text(input_bytes).unwrap();
            if text.len() < 2 || !text.starts_with('\\') {
                panic!("Invalid backslash escape: {}", text);
            }
            let content = &text[1..]; // remove the leading backslash
            PandocNativeIntermediate::IntermediateBaseText(content.to_string(), node_location(node))
        }
        "minus_metadata" => {
            let text = node.utf8_text(input_bytes).unwrap();
            PandocNativeIntermediate::IntermediateMetadataString(
                text.to_string(),
                node_location(node),
            )
        }
        "uri_autolink" => {
            // This is a URI autolink, we need to extract the content
            // by removing the angle brackets
            let text = node.utf8_text(input_bytes).unwrap();
            if text.len() < 2 || !text.starts_with('<') || !text.ends_with('>') {
                panic!("Invalid URI autolink: {}", text);
            }
            let content = &text[1..text.len() - 1]; // remove the angle brackets
            let mut attr = ("".to_string(), vec![], HashMap::new());
            // pandoc adds the class "uri" to autolinks
            attr.1.push("uri".to_string());
            PandocNativeIntermediate::IntermediateInline(Inline::Link(Link {
                content: vec![Inline::Str(Str {
                    text: content.to_string(),
                })],
                attr,
                target: (content.to_string(), "".to_string()),
            }))
        }
        "pipe_table_delimiter_cell" => {
            let mut has_starter_colon = false;
            let mut has_ending_colon = false;
            for (node, _) in children {
                if node == "pipe_table_align_right" {
                    has_ending_colon = true;
                } else if node == "pipe_table_align_left" {
                    has_starter_colon = true;
                } else if node == "-" {
                    continue;
                } else {
                    panic!("Unexpected node in pipe_table_delimiter_cell: {}", node);
                }
            }
            return PandocNativeIntermediate::IntermediatePipeTableDelimiterCell(
                match (has_starter_colon, has_ending_colon) {
                    (true, true) => Alignment::Center,
                    (true, false) => Alignment::Left,
                    (false, true) => Alignment::Right,
                    (false, false) => Alignment::Default,
                },
            );
        }
        "pipe_table_header" | "pipe_table_row" => {
            let mut row = Row {
                attr: empty_attr(),
                cells: Vec::new(),
            };
            for (node, child) in children {
                if node == "|" {
                    // This is a marker node, we don't need to do anything with it
                    continue;
                } else if node == "pipe_table_cell" {
                    if let PandocNativeIntermediate::IntermediateCell(cell) = child {
                        row.cells.push(cell);
                    } else {
                        panic!("Expected Cell in pipe_table_row, got {:?}", child);
                    }
                } else {
                    panic!(
                        "Expected pipe_table_cell in pipe_table_row, got {:?} {:?}",
                        node, child
                    );
                }
            }
            PandocNativeIntermediate::IntermediateRow(row)
        }
        "pipe_table_delimiter_row" => {
            // This is a row of delimiters, we don't need to do anything with it
            // but we need to return an empty row
            PandocNativeIntermediate::IntermediatePipeTableDelimiterRow(
                children
                    .into_iter()
                    .filter(|(node, _)| node != "|") // skip the marker nodes
                    .map(|(node, child)| match child {
                        PandocNativeIntermediate::IntermediatePipeTableDelimiterCell(alignment) => {
                            alignment
                        }
                        _ => panic!(
                            "Unexpected node in pipe_table_delimiter_row: {} {:?}",
                            node, child
                        ),
                    })
                    .collect(),
            )
        }
        "pipe_table_cell" => {
            let mut plain_content: Inlines = Vec::new();
            let mut table_cell = Cell {
                alignment: Alignment::Left,
                col_span: 1,
                row_span: 1,
                attr: ("".to_string(), vec![], HashMap::new()),
                content: vec![],
            };
            for (node, child) in children {
                if node == "inline" {
                    match child {
                        PandocNativeIntermediate::IntermediateInlines(inlines) => {
                            plain_content.extend(inlines);
                        }
                        _ => panic!("Expected Inlines in pipe_table_cell, got {:?}", child),
                    }
                } else {
                    panic!(
                        "Expected Inlines in pipe_table_cell, got {:?} {:?}",
                        node, child
                    );
                }
            }
            table_cell.content.push(Block::Plain(Plain {
                content: plain_content,
                filename: None,
                range: node_location(node),
            }));
            PandocNativeIntermediate::IntermediateCell(table_cell)
        }
        "pipe_table" => {
            let attr = empty_attr();
            let mut header: Option<Row> = None;
            let mut colspec: Vec<ColSpec> = Vec::new();
            let mut rows: Vec<Row> = Vec::new();
            for (node, child) in children {
                if node == "pipe_table_header" {
                    if let PandocNativeIntermediate::IntermediateRow(row) = child {
                        header = Some(row);
                    } else {
                        panic!("Expected Row in pipe_table_header, got {:?}", child);
                    }
                } else if node == "pipe_table_delimiter_row" {
                    match child {
                        PandocNativeIntermediate::IntermediatePipeTableDelimiterRow(row) => {
                            row.into_iter().for_each(|alignment| {
                                colspec.push((alignment, ColWidth::Default));
                            });
                        }
                        _ => panic!(
                            "Expected PipeTableDelimiterRow in pipe_table_delimiter_row, got {:?}",
                            child
                        ),
                    }
                } else if node == "pipe_table_row" {
                    if let PandocNativeIntermediate::IntermediateRow(row) = child {
                        rows.push(row);
                    } else {
                        panic!("Expected Row in pipe_table_row, got {:?}", child);
                    }
                } else {
                    panic!("Unexpected node in pipe_table: {}", node);
                }
            }
            PandocNativeIntermediate::IntermediateBlock(Block::Table(Table {
                attr,
                caption: Caption {
                    short: None,
                    long: None,
                },
                colspec,
                head: TableHead {
                    attr: empty_attr(),
                    rows: vec![header.unwrap()],
                },
                bodies: vec![TableBody {
                    attr: empty_attr(),
                    rowhead_columns: 0,
                    head: vec![],
                    body: rows,
                }],
                foot: TableFoot {
                    attr: empty_attr(),
                    rows: vec![],
                },
                filename: None,
                range: node_location(node),
            }))
        }
        "setext_h1_underline" => PandocNativeIntermediate::IntermediateSetextHeadingLevel(1),
        "setext_h2_underline" => PandocNativeIntermediate::IntermediateSetextHeadingLevel(2),
        "setext_heading" => {
            let mut content = Vec::new();
            let mut level = 1;
            for (_, child) in children {
                match child {
                    PandocNativeIntermediate::IntermediateBlock(Block::Paragraph(Paragraph {
                        content: inner_content,
                        ..
                    })) => {
                        content = inner_content;
                    }
                    PandocNativeIntermediate::IntermediateSetextHeadingLevel(l) => {
                        level = l;
                    }
                    _ => {
                        writeln!(
                            buf,
                            "[setext_heading] Warning: Unhandled node kind: {}",
                            node.kind()
                        )
                        .unwrap();
                    }
                }
            }
            return PandocNativeIntermediate::IntermediateBlock(Block::Header(Header {
                level,
                attr: empty_attr(),
                content,
                filename: None,
                range: node_location(node),
            }));
        }
        _ => {
            writeln!(
                buf,
                "[TOP-LEVEL MISSING NODE] Warning: Unhandled node kind: {}",
                node.kind()
            )
            .unwrap();
            let range = node_location(node);
            PandocNativeIntermediate::IntermediateUnknown(range)
        }
    }
}

fn shortcode_value_span(str: String) -> Inline {
    let mut attr_hash = HashMap::new();
    attr_hash.insert("data-raw".to_string(), str.clone());
    attr_hash.insert("data-value".to_string(), str);
    attr_hash.insert("data-is-shortcode".to_string(), "1".to_string());

    Inline::Span(Span {
        attr: (
            "".to_string(),
            vec!["quarto-shortcode__-param".to_string()],
            attr_hash,
        ),
        content: vec![],
    })
}

fn shortcode_key_value_span(key: String, value: String) -> Inline {
    let mut attr_hash = HashMap::new();

    // this needs to be fixed and needs to use the actual source. We'll do that when we have source mapping
    attr_hash.insert(
        "data-raw".to_string(),
        format!("{} = {}", key.clone(), value.clone()),
    );
    attr_hash.insert("data-key".to_string(), key);
    attr_hash.insert("data-value".to_string(), value);
    attr_hash.insert("data-is-shortcode".to_string(), "1".to_string());

    Inline::Span(Span {
        attr: (
            "".to_string(),
            vec!["quarto-shortcode__-param".to_string()],
            attr_hash,
        ),
        content: vec![],
    })
}

fn shortcode_to_span(shortcode: Shortcode) -> Span {
    let mut attr_hash: HashMap<String, String> = HashMap::new();
    let mut content: Inlines = vec![shortcode_value_span(shortcode.name)];
    for arg in shortcode.positional_args {
        match arg {
            ShortcodeArg::String(text) => {
                content.push(shortcode_value_span(text));
            }
            ShortcodeArg::Number(num) => {
                content.push(shortcode_value_span(num.to_string()));
            }
            ShortcodeArg::Boolean(b) => {
                content.push(shortcode_value_span(if b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }));
            }
            ShortcodeArg::Shortcode(inner_shortcode) => {
                content.push(Inline::Span(shortcode_to_span(inner_shortcode)));
            }
            ShortcodeArg::KeyValue(spec) => {
                for (key, value) in spec {
                    match value {
                        ShortcodeArg::String(text) => {
                            content.push(shortcode_key_value_span(key, text));
                        }
                        ShortcodeArg::Number(num) => {
                            content.push(shortcode_key_value_span(key, num.to_string()));
                        }
                        ShortcodeArg::Boolean(b) => {
                            content.push(shortcode_key_value_span(
                                key,
                                if b {
                                    "true".to_string()
                                } else {
                                    "false".to_string()
                                },
                            ));
                        }
                        ShortcodeArg::Shortcode(_) => {
                            eprintln!("PANIC - Quarto doesn't support nested shortcodes");
                            std::process::exit(1);
                        }
                        _ => {
                            panic!("Unexpected ShortcodeArg type in shortcode: {:?}", value);
                        }
                    }
                }
            }
        }
    }
    attr_hash.insert("data-is-shortcode".to_string(), "1".to_string());
    Span {
        attr: (
            "".to_string(),
            vec!["quarto-shortcode__".to_string()],
            attr_hash,
        ),
        content,
    }
}

fn trim_inlines(inlines: Inlines) -> (Inlines, bool) {
    let mut result: Inlines = Vec::new();
    let mut at_start = true;
    let mut space_run: Inlines = Vec::new();
    let mut changed = false;
    for inline in inlines {
        match &inline {
            Inline::Space(_) if at_start => {
                // skip leading spaces
                changed = true;
                continue;
            }
            Inline::Space(_) => {
                // collect spaces
                space_run.push(inline);
                continue;
            }
            _ => {
                result.extend(space_run.drain(..));
                result.push(inline);
                at_start = false;
            }
        }
    }
    if space_run.len() > 0 {
        changed = true;
    }
    (result, changed)
}

pub fn desugar(doc: Pandoc) -> Result<Pandoc, Vec<String>> {
    let mut errors = Vec::new();
    let raw_reader_format_specifier = Regex::new(r"<(?P<reader>.+)").unwrap();
    let result = {
        let mut filter = Filter::new()
            .with_superscript(|mut superscript| {
                let (content, changed) = trim_inlines(superscript.content);
                if !changed {
                    return Unchanged(Superscript {
                        content,
                        ..superscript
                    });
                } else {
                    superscript.content = content;
                    FilterResult(vec![Inline::Superscript(superscript)], true)
                }
            })
            // add attribute to headers that have them.
            .with_header(|mut header| {
                let is_last_attr = header
                    .content
                    .last()
                    .map_or(false, |v| matches!(v, Inline::Attr(_)));
                if !is_last_attr {
                    return Unchanged(header);
                }

                let Some(Inline::Attr(attr)) = header.content.pop() else {
                    panic!("shouldn't happen, header should have an attribute at this point");
                };
                header.attr = attr;
                header.content = trim_inlines(header.content).0;
                FilterResult(vec![Block::Header(header)], true)
            })
            // attempt to desugar single-image paragraphs into figures
            .with_paragraph(|para| {
                if para.content.len() != 1 {
                    return Unchanged(para);
                }
                let first = para.content.first().unwrap();
                let Inline::Image(image) = first else {
                    return Unchanged(para);
                };
                if image.content.is_empty() {
                    return Unchanged(para);
                }
                let figure_attr: Attr = (image.attr.0.clone(), vec![], HashMap::new());
                let image_attr: Attr = ("".to_string(), image.attr.1.clone(), image.attr.2.clone());
                let mut new_image = image.clone();
                new_image.attr = image_attr;
                // FIXME all source location is broken here
                FilterResult(
                    vec![Block::Figure(Figure {
                        attr: figure_attr,
                        caption: Caption {
                            short: None,
                            long: Some(vec![Block::Plain(Plain {
                                content: image.content.clone(),
                                filename: None,
                                range: empty_range(),
                            })]),
                        },
                        content: vec![Block::Plain(Plain {
                            content: vec![Inline::Image(new_image)],
                            filename: None,
                            range: empty_range(),
                        })],
                        filename: None,
                        range: empty_range(),
                    })],
                    true,
                )
            })
            .with_shortcode(|shortcode| {
                FilterResult(vec![Inline::Span(shortcode_to_span(shortcode))], false)
            })
            .with_note_reference(|note_ref| {
                let mut kv = HashMap::new();
                kv.insert("reference-id".to_string(), note_ref.id);
                FilterResult(
                    vec![Inline::Span(Span {
                        attr: (
                            "".to_string(),
                            vec!["quarto-note-reference".to_string()],
                            kv,
                        ),
                        content: vec![],
                    })],
                    false,
                )
            })
            .with_inlines(|inlines| {
                let mut result = vec![];
                // states in this state machine:
                // 0. normal state, where we just collect inlines
                // 1. we just saw a valid cite (only one citation, no prefix or suffix)
                // 2. from 1, we then saw a space
                // 3. from 2, we then saw a span with only Strs and Spaces.
                //
                //    Here, we emit the cite and add the span content to the cite suffix.
                let mut state = 0;
                let mut pending_cite: Option<Cite> = None;

                for inline in inlines {
                    match state {
                        0 => {
                            // Normal state - check if we see a valid cite
                            if let Inline::Cite(cite) = &inline {
                                if cite.citations.len() == 1
                                    && cite.citations[0].prefix.is_empty()
                                    && cite.citations[0].suffix.is_empty()
                                {
                                    // Valid cite - transition to state 1
                                    state = 1;
                                    pending_cite = Some(cite.clone());
                                } else {
                                    // Not a simple cite, just add it
                                    result.push(inline);
                                }
                            } else {
                                result.push(inline);
                            }
                        }
                        1 => {
                            // Just saw a valid cite - check for space
                            if let Inline::Space(_) = inline {
                                // Transition to state 2
                                state = 2;
                            } else {
                                // Not a space, emit pending cite and reset
                                if let Some(cite) = pending_cite.take() {
                                    result.push(Inline::Cite(cite));
                                }
                                result.push(inline);
                                state = 0;
                            }
                        }
                        2 => {
                            // After cite and space - check for span with only Strs and Spaces
                            if let Inline::Span(span) = &inline {
                                // Check if span contains only Str and Space inlines
                                let is_valid_suffix = span
                                    .content
                                    .iter()
                                    .all(|i| matches!(i, Inline::Str(_) | Inline::Space(_)));

                                if is_valid_suffix {
                                    // State 3 - merge span content into cite suffix
                                    if let Some(mut cite) = pending_cite.take() {
                                        // Add span content to the citation's suffix
                                        cite.citations[0].suffix = span.content.clone();
                                        result.push(Inline::Cite(cite));
                                    }
                                    state = 0;
                                } else {
                                    // Invalid span, emit pending cite, space, and span
                                    if let Some(cite) = pending_cite.take() {
                                        result.push(Inline::Cite(cite));
                                    }
                                    result.push(Inline::Space(Space {
                                        filename: None,
                                        range: empty_range(),
                                    }));
                                    result.push(inline);
                                    state = 0;
                                }
                            } else {
                                // Not a span, emit pending cite, space, and current inline
                                if let Some(cite) = pending_cite.take() {
                                    result.push(Inline::Cite(cite));
                                }
                                result.push(Inline::Space(Space {
                                    filename: None,
                                    range: empty_range(),
                                }));
                                result.push(inline);
                                state = 0;
                            }
                        }
                        _ => unreachable!("Invalid state: {}", state),
                    }
                }

                // Handle any pending cite at the end
                if let Some(cite) = pending_cite {
                    result.push(Inline::Cite(cite));
                    if state == 2 {
                        result.push(Inline::Space(Space {
                            filename: None,
                            range: empty_range(),
                        }));
                    }
                }

                FilterResult(result, true)
            })
            .with_raw_block(move |raw_block| {
                let Some(captures) = raw_reader_format_specifier.captures(&raw_block.text) else {
                    return Unchanged(raw_block);
                };
                return FilterResult(
                    vec![Block::RawBlock(RawBlock {
                        format: "pandoc-reader:".to_string() + &captures["reader"],
                        ..raw_block
                    })],
                    false,
                );
            })
            .with_attr(|attr| {
                // TODO in order to do good error messages here, attr will need source mapping
                errors.push(format!(
                    "Found attr in desugar: {:?} - this should have been removed",
                    attr
                ));
                FilterResult(vec![], false)
            });
        topdown_traverse(doc, &mut filter)
    };
    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(result)
    }
}

pub fn treesitter_to_pandoc<T: Write>(
    buf: &mut T,
    tree: &tree_sitter_qmd::MarkdownTree,
    input_bytes: &[u8],
) -> Result<Pandoc, Vec<String>> {
    let result = bottomup_traverse_concrete_tree(
        &mut tree.walk(),
        &mut |node, children, input_bytes| native_visitor(buf, node, children, input_bytes),
        &input_bytes,
    );
    let (_, PandocNativeIntermediate::IntermediatePandoc(pandoc)) = result else {
        panic!("Expected Pandoc, got {:?}", result)
    };
    desugar(pandoc)
}
