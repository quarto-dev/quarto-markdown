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
type Attr = (String, Vec<String>, HashMap<String, String>);

type Blocks = Vec<Block>;
type Inlines = Vec<Inline>;

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

type ListAttributes = (usize, ListNumberStyle, ListNumberDelim);

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

type ColSpec = (Alignment, ColWidth);

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

type Target = (String, String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MathType { InlineMath, DisplayMath }

#[derive(Debug, Clone, PartialEq)]
pub enum Inline {
    Str {
        text: String,
    },
    Emph {
        content: Inlines,
    },
    Underline {
        content: Inlines,
    },
    Strong {
        content: Inlines,
    },
    Strikeout {
        content: Inlines,
    },
    Superscript {
        content: Inlines,
    },
    Subscript {
        content: Inlines,
    },
    SmallCaps {
        content: Inlines,
    },
    Quoted {
        quote_type: QuoteType,
        content: Inlines,
    },
    Cite {
        citations: Vec<Citation>,
        content: Inlines,
    },
    Code {
        attr: Attr,
        text: String,
    },
    Space,
    SoftBreak,
    LineBreak,
    Math {
        math_type: MathType,
        text: String,
    },
    RawInline {
        format: String,
        text: String,
    },
    Link {
        attr: Attr,
        content: Inlines,
        target: Target,
    },
    Image {
        attr: Attr,
        content: Inlines,
        target: Target,
    },
    Note {
        content: Blocks,
    },
    Span {
        attr: Attr,
        content: Inlines,
    },
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
    IntermediateSection(Vec<Block>),
    IntermediateBlock(Block),
    IntermediateInline(Inline),
    IntermediateInlines(Vec<Inline>),
    IntermediateBaseText(String),
    IntermediateLatexInlineDelimiter,
    IntermediateLatexDisplayDelimiter,
    IntermediateUnknown,
}

fn native_visitor(node: &tree_sitter::Node, children: Vec<(String, PandocNativeIntermediate)>, input_bytes: &[u8]) -> PandocNativeIntermediate {

    let whitespace_re = Regex::new(r"\s+").unwrap();

    let native_inline = |(node, child)| {
        match child {
            PandocNativeIntermediate::IntermediateInline(inline) => inline,
            PandocNativeIntermediate::IntermediateBaseText(text) => {
                if let Some(_) = whitespace_re.find(&text) {
                    Inline::Space
                } else {
                    Inline::Str { text }
                }
            }
            _ => panic!("Expected Inline, got {:?} {:?}", node, child),
        }
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
            PandocNativeIntermediate::IntermediateInline(Inline::Emph {
                content: inlines,
            })
        },
        "strong_emphasis" => {
            let inlines: Vec<Inline> = children
                .into_iter()
                .filter(|(node, _)| {
                    node != "emphasis_delimiter" // skip emphasis delimiters
                }).map(native_inline).collect();
            PandocNativeIntermediate::IntermediateInline(Inline::Strong {
                content: inlines,
            })
        },
        "inline" => {
            let inlines: Vec<Inline> = children.into_iter().map(native_inline).collect();
            PandocNativeIntermediate::IntermediateInlines(inlines)
        },
        "emphasis_delimiter" => {
            // This is a marker node, we don't need to do anything with it
            PandocNativeIntermediate::IntermediateUnknown
        },
        "soft_line_break" => {
            PandocNativeIntermediate::IntermediateInline(Inline::SoftBreak)
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
        }
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
                    PandocNativeIntermediate::IntermediateInline(Inline::Math {
                        math_type: math_type,
                        text,
                    })
                }
                _ => panic!("Expected BaseText in latex_span, got {:?}", child),
            }
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