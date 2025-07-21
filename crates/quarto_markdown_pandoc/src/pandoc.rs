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

#[derive(Debug, Clone, PartialEq)]
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
pub enum Block {
    Plain {
        content: Inlines,
    },
    Paragraph {
        content: Inlines,
    },
    LineBlock {
        content: Vec<Inlines>,
    },
    CodeBlock {
        attr: Attr,
        text: String,
    },
    RawBlock {
        format: String,
        text: String,
    },
    BlockQuote {
        content: Blocks,
    },
    OrderedList {
        attr: ListAttributes,
        content: Vec<Blocks>,
    },
    BulletList {
        content: Vec<Blocks>,
    },
    DefinitionList {
        content: Vec<(Inlines, Vec<Blocks>)>,
    },
    Header {
        level: usize,
        attr: Attr,
        content: Inlines,
    },
    HorizontalRule,
    Table {
        attr: Attr,
        caption: Caption,
        colspec: Vec<ColSpec>,
        head: TableHead,
        bodies: Vec<TableBody>,
        foot: TableFoot,
    },
    Figure {
        attr: Attr,
        caption: Caption,
        content: Blocks,
    },
    Div {
        attr: Attr,
        content: Blocks,
    }
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
pub struct Space {}


#[derive(Debug, Clone, PartialEq)]
pub struct LineBreak {}

#[derive(Debug, Clone, PartialEq)]
pub struct SoftBreak {}

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
    IntermediateBaseText(String),
    IntermediateLatexInlineDelimiter,
    IntermediateLatexDisplayDelimiter,
    IntermediateKeyValueSpec(HashMap<String, String>),
    IntermediateRawFormat(String),
    IntermediateUnknown,
}

fn is_empty_attr(attr: &Attr) -> bool {
    attr.0.is_empty() && attr.1.is_empty() && attr.2.is_empty()
}

fn is_empty_target(target: &Target) -> bool {
    target.0.is_empty() && target.1.is_empty()
}

fn native_visitor(node: &tree_sitter::Node, children: Vec<(String, PandocNativeIntermediate)>, input_bytes: &[u8]) -> PandocNativeIntermediate {

    let whitespace_re = Regex::new(r"\s+").unwrap();
    let escaped_double_quote_re = Regex::new("[\\\\][\"]").unwrap();
    let escaped_single_quote_re = Regex::new("[\\\\][']").unwrap();

    let native_inline = |(node, child)| {
        match child {
            PandocNativeIntermediate::IntermediateInline(inline) => inline,
            PandocNativeIntermediate::IntermediateBaseText(text) => {
                if let Some(_) = whitespace_re.find(&text) {
                    Inline::Space(Space {})
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
                PandocNativeIntermediate::IntermediateBaseText(text) => {
                    if let Some(_) = whitespace_re.find(&text) {
                        inlines.push(Inline::Space(Space {}))
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
            PandocNativeIntermediate::IntermediateBlock(Block::Paragraph {
                content: inlines,
            })
        },
        "commonmark_attribute" => {
            let mut attr = ("".to_string(), vec![], HashMap::new());
            children.into_iter().for_each(|(node, child)| {
                match child {
                    PandocNativeIntermediate::IntermediateBaseText(id) => {
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
                    PandocNativeIntermediate::IntermediateUnknown => {},
                    _ => panic!("Unexpected child in commonmark_attribute: {:?}", child),
                }
            });
            PandocNativeIntermediate::IntermediateAttr(attr)
        },
        "class_specifier" => {
            let id = node.utf8_text(input_bytes).unwrap().to_string().split_off(1);
            PandocNativeIntermediate::IntermediateBaseText(id)
        },
        "id_specifier" => {
            let id = node.utf8_text(input_bytes).unwrap().to_string().split_off(1);
            PandocNativeIntermediate::IntermediateBaseText(id)
        },
        "key_value_key" => {
            let id = node.utf8_text(input_bytes).unwrap().to_string();
            PandocNativeIntermediate::IntermediateBaseText(id)
        },
        "key_value_value" => {
            let value = node.utf8_text(input_bytes).unwrap().to_string();
            if value.starts_with('"') && value.ends_with('"') {
                let value = value[1..value.len()-1].to_string();
                println!("Unescaping double quotes in value: {}", value);
                println!("unescaped: {}", escaped_double_quote_re.replace_all(&value, "\""));
                PandocNativeIntermediate::IntermediateBaseText(
                    escaped_double_quote_re.replace_all(&value, "\"").to_string()
                )
            } else if value.starts_with('\'') && value.ends_with('\'') {
                let value = value[1..value.len()-1].to_string();
                PandocNativeIntermediate::IntermediateBaseText(
                    escaped_single_quote_re.replace_all(&value, "'").to_string()
                )
            } else {
                // If not quoted, return as is
                PandocNativeIntermediate::IntermediateBaseText(value)
            }
        },
        "link_title" => {
            let title = node.utf8_text(input_bytes).unwrap().to_string();
            let title = title[1..title.len()-1].to_string();
            PandocNativeIntermediate::IntermediateBaseText(title)
        },
        "link_destination" => {
            let url = node.utf8_text(input_bytes).unwrap().to_string();
            PandocNativeIntermediate::IntermediateBaseText(url)
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
                    PandocNativeIntermediate::IntermediateBaseText(text) => {
                        if node == "link_destination" {
                            target.0 = text; // URL
                        } else if node == "link_title" {
                            target.1 = text; // Title
                        } else {
                            panic!("Unexpected image node: {}", node);
                        }
                    },
                    PandocNativeIntermediate::IntermediateUnknown => {},
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
                    PandocNativeIntermediate::IntermediateBaseText(text) => {
                        if node == "link_destination" {
                            target.0 = text; // URL
                        } else if node == "link_title" {
                            target.1 = text; // Title
                        } else {
                            panic!("Unexpected inline_link node: {}", node);
                        }
                    },
                    PandocNativeIntermediate::IntermediateUnknown => {},
                    PandocNativeIntermediate::IntermediateInlines(inlines) => content.extend(inlines),
                    PandocNativeIntermediate::IntermediateInline(inline) => content.push(inline),
                    _ => panic!("Unexpected child in inline_link: {:?}", child),
                }
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
                    let mut new_attr = (attr.0.clone(), attr.1.clone(), attr.2.clone());
                    new_attr.1 = new_attr.1.into_iter().filter(|s| s != "smallcaps").collect();
                    if is_empty_attr(&new_attr) {
                        return Inline::SmallCaps(SmallCaps { content });
                    }
                    let inner_inline = make_span_inline(new_attr, target, content);
                    return Inline::SmallCaps(SmallCaps { content: vec![inner_inline] });
                } else if attr.1.contains(&"ul".to_string()) {
                    let mut new_attr = (attr.0.clone(), attr.1.clone(), attr.2.clone());
                    new_attr.1 = new_attr.1.into_iter().filter(|s| s != "ul").collect();
                    if is_empty_attr(&new_attr) {
                        return Inline::Underline(Underline { content });
                    }
                    let inner_inline = make_span_inline(new_attr, target, content);
                    return Inline::Underline(Underline { content: vec![inner_inline] });
                } else if attr.1.contains(&"underline".to_string()) {
                    let mut new_attr = (attr.0.clone(), attr.1.clone(), attr.2.clone());
                    new_attr.1 = new_attr.1.into_iter().filter(|s| s != "underline").collect();
                    if is_empty_attr(&new_attr) {
                        return Inline::Underline(Underline { content });
                    }
                    let inner_inline = make_span_inline(new_attr, target, content);
                    return Inline::Underline(Underline { content: vec![inner_inline] });
                }

                return Inline::Span(Span { attr, content });
            }

            return PandocNativeIntermediate::IntermediateInline(
                make_span_inline(attr, target, content)
            );
        },
        "key_value_specifier" => {
            let mut spec = HashMap::new();
            let mut current_key: Option<String> = None;
            for (node, child) in children {
                if let PandocNativeIntermediate::IntermediateBaseText(value) = child {
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
            let raw = node.utf8_text(input_bytes).unwrap().to_string();
            PandocNativeIntermediate::IntermediateBaseText(raw[1..].to_string())            
        },
        "code_content" |
        "latex_content" |
        "text_base" => {
            node.utf8_text(input_bytes)
                .map(|text| PandocNativeIntermediate::IntermediateBaseText(text.to_string()))
                .unwrap()
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
        "code_span_delimiter" |
        "single_quoted_span_delimiter" |
        "double_quoted_span_delimiter" |
        "emphasis_delimiter" => {
            // This is a marker node, we don't need to do anything with it
            PandocNativeIntermediate::IntermediateUnknown
        },
        "soft_line_break" => {
            PandocNativeIntermediate::IntermediateInline(Inline::SoftBreak(SoftBreak { }))
        },
        "hard_line_break" => {
            PandocNativeIntermediate::IntermediateInline(Inline::LineBreak(LineBreak { }))
        },
        "latex_span_delimiter" => {
            let str = node.utf8_text(input_bytes).unwrap();
            if str == "$" {
                PandocNativeIntermediate::IntermediateLatexInlineDelimiter
            } else if str == "$$" {
                PandocNativeIntermediate::IntermediateLatexDisplayDelimiter
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
                content: vec![Block::Paragraph { content: inlines }]
            }))
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
                            PandocNativeIntermediate::IntermediateBaseText(_) => true,
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
                .map(|(node, child)| {
                    match child {
                        PandocNativeIntermediate::IntermediateAttr(a) => {
                            attr = a;
                            // IntermediateUnknown here "consumes" the node
                            (node, PandocNativeIntermediate::IntermediateUnknown)
                        }
                        PandocNativeIntermediate::IntermediateRawFormat(raw) => {
                            is_raw = Some(raw);
                            // IntermediateUnknown here "consumes" the node
                            (node, PandocNativeIntermediate::IntermediateUnknown)
                        }
                        _ => (node, child)
                    }
                })
                .filter(|(_, child)| {
                    *child != PandocNativeIntermediate::IntermediateUnknown
                }).collect();
            assert!(inlines.len() == 1, "Expected exactly one inline in code_span, got {}", inlines.len());
            let (_, child) = inlines.remove(0);
            match child {
                PandocNativeIntermediate::IntermediateBaseText(text) => {
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
                    if *child == PandocNativeIntermediate::IntermediateLatexInlineDelimiter {
                        is_inline_math = true;
                        false // skip the delimiter
                    } else if *child == PandocNativeIntermediate::IntermediateLatexDisplayDelimiter {
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
                PandocNativeIntermediate::IntermediateBaseText(text) => {
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
                match child {
                    PandocNativeIntermediate::IntermediateBaseText(raw) => {
                        return PandocNativeIntermediate::IntermediateRawFormat(raw);
                    }
                    _ => {}
                }
            }
            panic!("Expected raw_attribute to have a format, but found none");
        },
        _ => {
            eprintln!("Warning: Unhandled node kind: {}", node.kind());
            PandocNativeIntermediate::IntermediateUnknown
        }
    }
}

pub fn treesitter_to_pandoc(tree: &tree_sitter_qmd::MarkdownTree, input_bytes: &[u8]) -> Pandoc {
    let result = bottomup_traverse_concrete_tree(&mut tree.walk(), &mut native_visitor, &input_bytes);
    match result {
        (_, PandocNativeIntermediate::IntermediatePandoc(pandoc)) => pandoc,
        _ => panic!("Expected Pandoc, got {:?}", result),
    }
}