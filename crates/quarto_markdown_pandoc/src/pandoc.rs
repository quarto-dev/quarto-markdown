/*
 * pandoc.rs
 * Copyright (c) 2025 Posit, PBC
 */

/*
 * A data structure that mimics Pandoc's `data Pandoc` type.
 * This is used to represent the parsed structure of a Quarto Markdown document.
 */

use std::collections::HashMap;
use regex::Regex;

use crate::traversals::bottomup_traverse_concrete_tree;
use crate::filters::{topdown_traverse, Filter};

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
            column: start.column 
        },
        end: Location { 
            offset: node.end_byte(), 
            row: end.row, 
            column: end.column 
        }
    }
}

fn empty_range() -> Range {
    Range {
        start: Location { offset: 0, row: 0, column: 0 },
        end: Location { offset: 0, row: 0, column: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pandoc {
    pub blocks: Vec<Block>
    // eventually, meta: 
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
    Justified,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ColWidth {
    Default,
    Percentage(f64)
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph {
    pub content: Inlines,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineBlock {
    pub content: Vec<Inlines>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock {
    pub attr: Attr,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawBlock {
    pub format: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockQuote {
    pub content: Blocks,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderedList {
    pub attr: ListAttributes,
    pub content: Vec<Blocks>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BulletList {
    pub content: Vec<Blocks>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefinitionList {
    pub content: Vec<(Inlines, Vec<Blocks>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub level: usize,
    pub attr: Attr,
    pub content: Inlines,
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Figure {
    pub attr: Attr,
    pub caption: Caption,
    pub content: Blocks,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Div {
    pub attr: Attr,
    pub content: Blocks,
}

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuoteType { SingleQuote, DoubleQuote }

pub type Target = (String, String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MathType { InlineMath, DisplayMath }

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

impl SourceLocation for HorizontalRule {
    fn filename(&self) -> Option<String> {
        self.filename.clone()
    }
    
    fn range(&self) -> Range {
        self.range.clone()
    }
}

impl SourceLocation for Space {
    fn filename(&self) -> Option<String> {
        self.filename.clone()
    }
    
    fn range(&self) -> Range {
        self.range.clone()
    }
}

impl SourceLocation for LineBreak {
    fn filename(&self) -> Option<String> {
        self.filename.clone()
    }
    
    fn range(&self) -> Range {
        self.range.clone()
    }
}

impl SourceLocation for SoftBreak {
    fn filename(&self) -> Option<String> {
        self.filename.clone()
    }
    
    fn range(&self) -> Range {
        self.range.clone()
    }
}

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
    Shortcode(Shortcode),
    NoteReference(NoteReference)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Citation {
    pub id: String,
    pub prefix: Inlines,
    pub suffix: Inlines,
    pub mode: CitationMode,
    pub note_num: usize,
    pub hash: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitationMode {
    AuthorInText,
    SuppressAuthor,
    NormalCitation
}

#[derive(Debug, Clone, PartialEq)]
enum PandocNativeIntermediate {
    IntermediatePandoc(Pandoc),
    IntermediateAttr(Attr),
    IntermediateSection(Vec<Block>),
    IntermediateBlock(Block),
    IntermediateInline(Inline),
    IntermediateInlines(Vec<Inline>),
    IntermediateBaseText(String, Range),
    IntermediateLatexInlineDelimiter(Range),
    IntermediateLatexDisplayDelimiter(Range),
    IntermediateKeyValueSpec(HashMap<String, String>),
    IntermediateRawFormat(String, Range),
    IntermediateShortcodeArg(ShortcodeArg, Range),
    IntermediateUnknown(Range),
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
        new_attr.1 = new_attr.1.into_iter().filter(|s| s != "smallcaps").collect();
        if is_empty_attr(&new_attr) {
            return Inline::SmallCaps(SmallCaps { content });
        }
        let inner_inline = make_span_inline(new_attr, target, content);
        return Inline::SmallCaps(SmallCaps { content: vec![inner_inline] });
    } else if attr.1.contains(&"ul".to_string()) {
        let mut new_attr = attr.clone();
        new_attr.1 = new_attr.1.into_iter().filter(|s| s != "ul").collect();
        if is_empty_attr(&new_attr) {
            return Inline::Underline(Underline { content });
        }
        let inner_inline = make_span_inline(new_attr, target, content);
        return Inline::Underline(Underline { content: vec![inner_inline] });
    } else if attr.1.contains(&"underline".to_string()) {
        let mut new_attr = attr.clone();
        new_attr.1 = new_attr.1.into_iter().filter(|s| s != "underline").collect();
        if is_empty_attr(&new_attr) {
            return Inline::Underline(Underline { content });
        }
        let inner_inline = make_span_inline(new_attr, target, content);
        return Inline::Underline(Underline { content: vec![inner_inline] });
    }

    return Inline::Span(Span { attr, content });
}

fn make_cite_inline(attr: Attr, target: Target, content: Inlines) -> Inline {
    
    // the traversal here is slightly inefficient because we need
    // to non-destructively check for the goodness of the content
    // before deciding to destructively create a Cite

    let is_semicolon = |inline: &Inline| {
        match &inline {
            Inline::Str(Str { text }) => text == ";",
            _ => false,
        }
    };

    let is_good_cite = content.split(is_semicolon).all(|slice| {
        slice.iter().any(|inline| {
            match inline {
                Inline::Cite(_) => true,
                _ => false,
            }
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
        .split(is_semicolon).map(|slice| {
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
            match cite {
                None => panic!("Cite inline should have at least one citation, found none"),
                Some(mut c) => {
                    if c.citations.len() != 1 {
                        panic!("Cite inline should have exactly one citation, found: {:?}", c.citations);
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
                }
            }
        }).collect();
    return Inline::Cite(Cite {
        citations,
        content: vec![],
    });               
}

fn native_visitor(node: &tree_sitter::Node, children: Vec<(String, PandocNativeIntermediate)>, input_bytes: &[u8]) -> PandocNativeIntermediate {

    let whitespace_re = Regex::new(r"\s+").unwrap();
    let escaped_double_quote_re = Regex::new("[\\\\][\"]").unwrap();
    let escaped_single_quote_re = Regex::new("[\\\\][']").unwrap();

    let node_text = || {
        node.utf8_text(input_bytes).unwrap().to_string()
    };

    let string_as_base_text = || {
        let location = node_location(node);
        let value = node_text();
        if value.starts_with('"') && value.ends_with('"') {
            let value = value[1..value.len()-1].to_string();
            println!("Unescaping double quotes in value: {}", value);
            println!("unescaped: {}", escaped_double_quote_re.replace_all(&value, "\""));
            PandocNativeIntermediate::IntermediateBaseText(
                escaped_double_quote_re.replace_all(&value, "\"").to_string(),
                location
            )
        } else if value.starts_with('\'') && value.ends_with('\'') {
            let value = value[1..value.len()-1].to_string();
            PandocNativeIntermediate::IntermediateBaseText(
                escaped_single_quote_re.replace_all(&value, "'").to_string(),
                location
            )
        } else {
            // If not quoted, return as is
            PandocNativeIntermediate::IntermediateBaseText(value, location)
        }
    };
    let native_inline = |(node, child)| {
        match child {
            PandocNativeIntermediate::IntermediateInline(inline) => inline,
            PandocNativeIntermediate::IntermediateBaseText(text, range) => {
                if let Some(_) = whitespace_re.find(&text) {
                    Inline::Space(Space { filename: None, range })
                } else {
                    Inline::Str(Str { text })
                }
            }
            _ => panic!("Expected Inline, got {:?} {:?}", node, child),
        }
    };
    let native_inlines = |children| {
        let mut inlines: Vec<Inline> = Vec::new();
        for (_, child) in children {
            match child {
                PandocNativeIntermediate::IntermediateInline(inline) => inlines.push(inline),
                PandocNativeIntermediate::IntermediateInlines(inner_inlines) => inlines.extend(inner_inlines),
                PandocNativeIntermediate::IntermediateBaseText(text, range) => {
                    if let Some(_) = whitespace_re.find(&text) {
                        inlines.push(Inline::Space(Space { filename: None, range }))
                    } else {
                        inlines.push(Inline::Str(Str { text }))
                    }
                }
                _ => panic!("Unexpected child in link_text: {:?}", child)
            }
        }
        inlines
    };

    match node.kind() {
        "note_reference_id" |
        "citation_id_suppress_author" |
        "citation_id_author_in_text" |
        "link_destination" |
        "key_value_key" |
        "code_content" |
        "latex_content" |
        "text_base" => {
            PandocNativeIntermediate::IntermediateBaseText(node_text(), node_location(node))
        },
        "document" => {
            let mut blocks: Vec<Block> = Vec::new();
            children.into_iter().for_each(|(_, child)| {
                if let PandocNativeIntermediate::IntermediateBlock(block) = child {
                    blocks.push(block);
                } else if let PandocNativeIntermediate::IntermediateSection(section) = child {
                    blocks.extend(section);
                } else {
                    panic!("Expected block or section, got {:?}", child);
                }
            });
            PandocNativeIntermediate::IntermediatePandoc (Pandoc { blocks })
        },
        "section" => {
            let blocks = children.into_iter().map(|(_, child)| {
                if let PandocNativeIntermediate::IntermediateBlock(block) = child {
                    block
                } else {
                    panic!("Expected Block, got {:?}", child);
                }
            }).collect();
            PandocNativeIntermediate::IntermediateSection(blocks)
        },
        "paragraph" => {
            let mut inlines: Vec<Inline> = Vec::new();
            for (_, child) in children {
                if let PandocNativeIntermediate::IntermediateInline(inline) = child {
                    inlines.push(inline);
                } else if let PandocNativeIntermediate::IntermediateInlines(inner_inlines) = child {
                    inlines.extend(inner_inlines);
                }
            }
            PandocNativeIntermediate::IntermediateBlock(Block::Paragraph(Paragraph {
                content: inlines,
            }))
        },
        "commonmark_attribute" => {
            let mut attr = ("".to_string(), vec![], HashMap::new());
            children.into_iter().for_each(|(node, child)| {
                match child {
                    PandocNativeIntermediate::IntermediateBaseText(id, _) => {
                        if node == "id_specifier" {
                            attr.0 = id;
                        } else if node == "class_specifier" {
                            attr.1.push(id);
                        } else {
                            panic!("Unexpected commonmark_attribute node: {}", node);
                        }
                    },
                    PandocNativeIntermediate::IntermediateKeyValueSpec(spec) => {
                        for (key, value) in spec {
                            attr.2.insert(key, value);
                        }
                    },
                    PandocNativeIntermediate::IntermediateUnknown(_) => {},
                    _ => panic!("Unexpected child in commonmark_attribute: {:?}", child),
                }
            });
            PandocNativeIntermediate::IntermediateAttr(attr)
        },
        "class_specifier" => {
            let id = node_text().split_off(1);
            PandocNativeIntermediate::IntermediateBaseText(id, node_location(node))
        },
        "id_specifier" => {
            let id = node_text().split_off(1);
            PandocNativeIntermediate::IntermediateBaseText(id, node_location(node))
        },
        "shortcode_naked_string" |
        "shortcode_name" => {
            let id = node_text().to_string();
            PandocNativeIntermediate::IntermediateShortcodeArg(ShortcodeArg::String(id), node_location(node))
        },
        "shortcode_string" => {
            match string_as_base_text() {
                PandocNativeIntermediate::IntermediateBaseText(id, _) => {
                    PandocNativeIntermediate::IntermediateShortcodeArg(ShortcodeArg::String(id), node_location(node))
                },
                _ => panic!("Expected BaseText in shortcode_string, got {:?}", string_as_base_text()),
            }
        }
        "key_value_value" => { string_as_base_text() },
        "link_title" => {
            let title = node_text();
            let title = title[1..title.len()-1].to_string();
            PandocNativeIntermediate::IntermediateBaseText(title, node_location(node))
        },
        "link_text" => {
            PandocNativeIntermediate::IntermediateInlines(native_inlines(children))
        },
        "image" => {
            let mut attr = ("".to_string(), vec![], HashMap::new());
            let mut target: Target = ("".to_string(), "".to_string());
            let mut content: Vec<Inline> = Vec::new();
            for (node, child) in children {
                if node == "image_description" {
                    match child {
                        PandocNativeIntermediate::IntermediateInlines(inlines) => {
                            content.extend(inlines);
                        },
                        _ => panic!("Expected inlines in image_description, got {:?}", child),
                    }
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
                    },
                    PandocNativeIntermediate::IntermediateUnknown(_) => {},
                    PandocNativeIntermediate::IntermediateInlines(inlines) => content.extend(inlines),
                    PandocNativeIntermediate::IntermediateInline(inline) => content.push(inline),
                    _ => panic!("Unexpected child in inline_link: {:?}", child),
                }
            }
            PandocNativeIntermediate::IntermediateInline(Inline::Image(Image {
                attr,
                content,
                target
            }))
        },
        "image_description" => {
            PandocNativeIntermediate::IntermediateInlines(native_inlines(children))
        },
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
                    },
                    PandocNativeIntermediate::IntermediateUnknown(_) => {},
                    PandocNativeIntermediate::IntermediateInlines(inlines) => content.extend(inlines),
                    PandocNativeIntermediate::IntermediateInline(inline) => content.push(inline),
                    _ => panic!("Unexpected child in inline_link: {:?}", child),
                }
            }
            let has_citations = content.iter().any(|inline| matches!(inline, Inline::Cite(_)));

            // an inline link might be a Cite if it has citations, no destination, and no title
            // and no attributes
            let is_cite = has_citations && is_empty_target(&target) && is_empty_attr(&attr);
            
            return PandocNativeIntermediate::IntermediateInline(
                if is_cite { 
                    make_cite_inline(attr, target, content)
                } else {
                    make_span_inline(attr, target, content) 
                }
            );
        },
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
                        eprintln!("Unexpected key_value_specifier node: {}", node);
                    }
                }
            }
            PandocNativeIntermediate::IntermediateKeyValueSpec(spec)
        },
        "raw_specifier" => {
            // like code_content but skipping first character
            let raw = node_text();
            PandocNativeIntermediate::IntermediateBaseText(raw[1..].to_string(), node_location(node))            
        },
        "emphasis" => {
            let inlines: Vec<Inline> = children
                .into_iter()
                .filter(|(node, _)| {
                    node != "emphasis_delimiter" // skip emphasis delimiters
                }).map(native_inline).collect();
            PandocNativeIntermediate::IntermediateInline(Inline::Emph(Emph {
                content: inlines,
            }))
        },
        "strong_emphasis" => {
            let inlines: Vec<Inline> = children
                .into_iter()
                .filter(|(node, _)| {
                    node != "emphasis_delimiter" // skip emphasis delimiters
                }).map(native_inline).collect();
            PandocNativeIntermediate::IntermediateInline(Inline::Strong(Strong {
                content: inlines,
            }))
        },
        "inline" => {
            let inlines: Vec<Inline> = children.into_iter().map(native_inline).collect();
            PandocNativeIntermediate::IntermediateInlines(inlines)
        },
        "citation" => {
            let mut citation_type = CitationMode::NormalCitation;
            let mut citation_id = String::new();
            for (node, child) in children {
                if node == "citation_id_suppress_author" {
                    citation_type = CitationMode::SuppressAuthor;
                    if let PandocNativeIntermediate::IntermediateBaseText(id, _) = child {
                        citation_id = id;
                    } else {
                        panic!("Expected BaseText in citation_id_suppress_author, got {:?}", child);
                    }
                } else if node == "citation_id_author_in_text" {
                    citation_type = CitationMode::AuthorInText;
                    if let PandocNativeIntermediate::IntermediateBaseText(id, _) = child {
                        citation_id = id;
                    } else {
                        panic!("Expected BaseText in citation_id_author_in_text, got {:?}", child);
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
                content: vec![
                    Inline::Str(Str {
                        text: node_text()
                    })
                ],
            }))
        },
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
        },
        "shortcode" |
        "shortcode_escaped" => {
            let is_escaped = node.kind() == "shortcode_escaped";
            let mut name = String::new();
            let mut positional_args: Vec<ShortcodeArg> = Vec::new();
            let mut keyword_args: HashMap<String, ShortcodeArg> = HashMap::new();
            for (node, child) in children {
                match (node.as_str(), child) {
                    ("shortcode_name", PandocNativeIntermediate::IntermediateShortcodeArg(ShortcodeArg::String(text), _)) |
                    ("shortcode_string", PandocNativeIntermediate::IntermediateShortcodeArg(ShortcodeArg::String(text), _)) => {
                        if name.is_empty() {
                            name = text;
                        } else {
                            positional_args.push(ShortcodeArg::String(text));
                        }
                    },
                    ("shortcode_keyword_param", PandocNativeIntermediate::IntermediateShortcodeArg(ShortcodeArg::KeyValue(spec), _)) => {
                        for (key, value) in spec {
                            keyword_args.insert(key, value);
                        }
                    },
                    ("shortcode", PandocNativeIntermediate::IntermediateInline(Inline::Shortcode(arg))) => {
                        positional_args.push(ShortcodeArg::Shortcode(arg));
                    },
                    ("shortcode_number", PandocNativeIntermediate::IntermediateShortcodeArg(arg, _)) |
                    ("shortcode_boolean", PandocNativeIntermediate::IntermediateShortcodeArg(arg, _)) => {
                        positional_args.push(arg);
                    },
                    ("shortcode_delimiter", _) => {
                        // This is a marker node, we don't need to do anything with it
                    },
                    (_, child) => panic!("Unexpected shortcode_escaped node: {} with child {:?}", node, child.clone()),
                }
            }
            PandocNativeIntermediate::IntermediateInline(Inline::Shortcode(Shortcode {
                is_escaped,
                name,
                positional_args,
                keyword_args,
            }))
        },
        "shortcode_keyword_param" => {
            let mut result = HashMap::new();
            let mut name = String::new();
            for (node, child) in children {
                match node.as_str() {
                    "shortcode_name" => {
                        match child {
                            PandocNativeIntermediate::IntermediateShortcodeArg(ShortcodeArg::String(text), _) => {
                                if name.is_empty() {
                                    name = text;
                                } else {
                                    result.insert(name.clone(), ShortcodeArg::String(text));
                                }
                            }
                            _ => panic!("Expected BaseText in shortcode_name, got {:?}", child)
                        }
                    },
                    "shortcode_string" | 
                    "shortcode_number" | "shortcode_naked_string" | "shortcode_boolean" => {
                        match child {
                            PandocNativeIntermediate::IntermediateShortcodeArg(arg, _) => {
                                result.insert(name.clone(), arg);
                            }
                            _ => panic!("Expected ShortcodeArg in shortcode_string, got {:?}", child)
                        }
                    },
                    _ => {
                        eprintln!("Warning: Unhandled node kind: {}", node);
                    }
                }
            }
            let range = node_location(node);
            PandocNativeIntermediate::IntermediateShortcodeArg(ShortcodeArg::KeyValue(result), range)
        },
        "shortcode_boolean" => {
            let value = node_text();
            let value = match value.as_str() {
                "true" => ShortcodeArg::Boolean(true),
                "false" => ShortcodeArg::Boolean(false),
                _ => panic!("Unexpected shortcode_boolean value: {}", value),
            };
            let range = node_location(node);
            PandocNativeIntermediate::IntermediateShortcodeArg(value, range)
        },
        "shortcode_number" => {
            let value = node_text();
            let range = node_location(node);
            match value.parse::<f64>() {
                Ok(num) => PandocNativeIntermediate::IntermediateShortcodeArg(ShortcodeArg::Number(num), range),
                Err(_) => panic!("Invalid shortcode_number: {}", value),
            }
        },
        "note_reference_delimiter" |
        "shortcode_delimiter" |
        "citation_delimiter" |
        "code_span_delimiter" |
        "single_quoted_span_delimiter" |
        "double_quoted_span_delimiter" |
        "superscript_delimiter" |
        "subscript_delimiter" |
        "strikeout_delimiter" |
        "emphasis_delimiter" => {
            // This is a marker node, we don't need to do anything with it
            let range = node_location(node);
            PandocNativeIntermediate::IntermediateUnknown(range)
        },
        "soft_line_break" => {
            PandocNativeIntermediate::IntermediateInline(Inline::SoftBreak(SoftBreak { 
                filename: None, 
                range: node_location(node) 
            }))
        },
        "hard_line_break" => {
            PandocNativeIntermediate::IntermediateInline(Inline::LineBreak(LineBreak { 
                filename: None, 
                range: node_location(node) 
            }))
        },
        "latex_span_delimiter" => {
            let str = node.utf8_text(input_bytes).unwrap();
            let range = node_location(node);
            if str == "$" {
                PandocNativeIntermediate::IntermediateLatexInlineDelimiter(range)
            } else if str == "$$" {
                PandocNativeIntermediate::IntermediateLatexDisplayDelimiter(range)
            } else {
                panic!("Warning: Unrecognized latex_span_delimiter: {}", str);
            }
        },
        "inline_note" => {
            let inlines: Vec<_> = children
                .into_iter()
                .filter(|(node, _)| node != "inline_note_delimiter")
                .map(native_inline)
                .collect();
            PandocNativeIntermediate::IntermediateInline(Inline::Note(Note {
                content: vec![Block::Paragraph(Paragraph { content: inlines })]
            }))
        },
        "superscript" => {
            let inlines: Vec<_> = children
                .into_iter()
                .filter(|(node, _)| node != "superscript_delimiter")
                .map(native_inline).collect();
            PandocNativeIntermediate::IntermediateInline(Inline::Superscript(Superscript {
                content: inlines }))
        },
        "subscript" => {
            let inlines: Vec<_> = children
                .into_iter()
                .filter(|(node, _)| node != "subscript_delimiter")
                .map(native_inline).collect();
            PandocNativeIntermediate::IntermediateInline(Inline::Subscript(Subscript {
                content: inlines }))
        },
        "strikeout" => {
            let inlines: Vec<_> = children
                .into_iter()
                .filter(|(node, _)| node != "strikeout_delimiter")
                .map(native_inline).collect();
            PandocNativeIntermediate::IntermediateInline(Inline::Strikeout(Strikeout {
                content: inlines }))
        },

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
                            _ => false
                        }
                    }
                }).map(native_inline).collect();
            PandocNativeIntermediate::IntermediateInline(Inline::Quoted(Quoted {
                quote_type,
                content: inlines }))
        },
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
                            (node_name, PandocNativeIntermediate::IntermediateUnknown(range))
                        }
                        PandocNativeIntermediate::IntermediateRawFormat(raw, _) => {
                            is_raw = Some(raw);
                            // IntermediateUnknown here "consumes" the node
                            (node_name, PandocNativeIntermediate::IntermediateUnknown(range))
                        }
                        _ => (node_name, child)
                    }
                })
                .filter(|(_, child)| {
                    match child {
                        PandocNativeIntermediate::IntermediateUnknown(_) => false, // skip unknown nodes
                        _ => true // keep other nodes
                    }
                }).collect();
            assert!(inlines.len() == 1, "Expected exactly one inline in code_span, got {}", inlines.len());
            let (_, child) = inlines.remove(0);
            match child {
                PandocNativeIntermediate::IntermediateBaseText(text, _) => {
                    if let Some(raw) = is_raw {
                        PandocNativeIntermediate::IntermediateInline(Inline::RawInline(RawInline {
                            format: raw,
                            text,
                        }))
                    } else {
                        PandocNativeIntermediate::IntermediateInline(Inline::Code(Code {
                            attr,
                            text,
                        }))
                    }
                }
                _ => panic!("Expected BaseText in code_span, got {:?}", child),
            }
        },
        "latex_span" => {
            let mut is_inline_math = false;
            let mut is_display_math = false;
            let mut inlines: Vec<_> = children
                .into_iter()
                .filter(|(_, child)| {
                    if matches!(child, PandocNativeIntermediate::IntermediateLatexInlineDelimiter(_)) {
                        is_inline_math = true;
                        false // skip the delimiter
                    } else if matches!(child, PandocNativeIntermediate::IntermediateLatexDisplayDelimiter(_)) {
                        is_display_math = true;
                        false // skip the delimiter
                    } else {
                        true // keep other nodes
                    }
                }).collect();
            assert!(inlines.len() == 1, "Expected exactly one inline in latex_span, got {}", inlines.len());
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
            match child {
                PandocNativeIntermediate::IntermediateBaseText(text, _) => {
                    PandocNativeIntermediate::IntermediateInline(Inline::Math(Math {
                        math_type: math_type,
                        text,
                    }))
                }
                _ => panic!("Expected BaseText in latex_span, got {:?}", child),
            }
        },
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
        },
        _ => {
            eprintln!("Warning: Unhandled node kind: {}", node.kind());
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
        attr: ("".to_string(), vec!["quarto-shortcode__-param".to_string()], attr_hash),
        content: vec![],
    })
}

fn shortcode_key_value_span(key: String, value: String) -> Inline {
    let mut attr_hash = HashMap::new();

    // this needs to be fixed and needs to use the actual source. We'll do that when we have source mapping
    attr_hash.insert("data-raw".to_string(), format!("{} = {}", key.clone(), value.clone()));
    attr_hash.insert("data-key".to_string(), key);
    attr_hash.insert("data-value".to_string(), value);
    attr_hash.insert("data-is-shortcode".to_string(), "1".to_string());

    Inline::Span(Span {
        attr: ("".to_string(), vec!["quarto-shortcode__-param".to_string()], attr_hash),
        content: vec![],
    })
}

fn shortcode_to_span(shortcode: Shortcode) -> Span {
    let mut attr_hash: HashMap<String, String> = HashMap::new();
    let mut content: Inlines = vec![
        shortcode_value_span(shortcode.name)
    ];
    for arg in shortcode.positional_args {
        match arg {
            ShortcodeArg::String(text) => {
                content.push(shortcode_value_span(text));
            }
            ShortcodeArg::Number(num) => {
                content.push(shortcode_value_span(num.to_string()));
            }
            ShortcodeArg::Boolean(b) => {
                content.push(shortcode_value_span(if b { "true".to_string() } else { "false".to_string() }));
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
                            content.push(shortcode_key_value_span(key, if b { "true".to_string() } else { "false".to_string() }));
                        }
                        ShortcodeArg::Shortcode(_) => {
                            panic!("Quarto itself doesn't yet support nested shortcodes");
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
        attr: ("".to_string(), vec!["quarto-shortcode__".to_string()], attr_hash),
        content
    }
}

pub fn desugar(doc: Pandoc) -> Pandoc {
    topdown_traverse(doc, &Filter {        
        shortcode: Some(|shortcode| {
            (vec![Inline::Span(shortcode_to_span(shortcode))], false)
        }),
        note_reference: Some(|note_ref| {
            let mut kv = HashMap::new();
            kv.insert("reference-id".to_string(), note_ref.id);
            (vec![Inline::Span(Span {
                attr: ("".to_string(), vec!["quarto-note-reference".to_string()], kv),
                content: vec![],
            })], false)
        }),
        inlines: Some(|inlines| {
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
                            if cite.citations.len() == 1 && 
                               cite.citations[0].prefix.is_empty() && 
                               cite.citations[0].suffix.is_empty() {
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
                    },
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
                    },
                    2 => {
                        // After cite and space - check for span with only Strs and Spaces
                        if let Inline::Span(span) = &inline {
                            // Check if span contains only Str and Space inlines
                            let is_valid_suffix = span.content.iter().all(|i| {
                                matches!(i, Inline::Str(_) | Inline::Space(_))
                            });
                            
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
                                result.push(Inline::Space(Space { filename: None, range: empty_range() }));
                                result.push(inline);
                                state = 0;
                            }
                        } else {
                            // Not a span, emit pending cite, space, and current inline
                            if let Some(cite) = pending_cite.take() {
                                result.push(Inline::Cite(cite));
                            }
                            result.push(Inline::Space(Space { filename: None, range: empty_range() }));
                            result.push(inline);
                            state = 0;
                        }
                    },
                    _ => unreachable!("Invalid state: {}", state),
                }
            }
            
            // Handle any pending cite at the end
            if let Some(cite) = pending_cite {
                result.push(Inline::Cite(cite));
                if state == 2 {
                    result.push(Inline::Space(Space { filename: None, range: empty_range() }));
                }
            }
            
            (result, true)
        }),
        ..Default::default()
    })
}

pub fn treesitter_to_pandoc(tree: &tree_sitter_qmd::MarkdownTree, input_bytes: &[u8]) -> Pandoc {
    let result = bottomup_traverse_concrete_tree(&mut tree.walk(), &mut native_visitor, &input_bytes);
    match result {
        (_, PandocNativeIntermediate::IntermediatePandoc(pandoc)) => desugar(pandoc),
        _ => panic!("Expected Pandoc, got {:?}", result),
    }
}