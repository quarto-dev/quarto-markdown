
/*
 * native.rs
 * Copyright (c) 2025 Posit, PBC
 */

use crate::pandoc::{Attr, Block, Citation, Inline, MathType, Pandoc, QuoteType, CitationMode};

fn write_safe_string(text: &str) -> String {
    format!("\"{}\"", text.replace("\\", "\\\\").replace("\"", "\\\""))
}

fn write_native_attr(attr: &Attr) -> String {
    let (id, classes, attrs) = attr;
    format!(
        "( {} , [{}] , [{}] )", 
        write_safe_string(&id), 
        classes
            .into_iter()
            .map(|str| write_safe_string(&str))
            .collect::<Vec<_>>()
            .join(", "),
        attrs.into_iter()
            .map(|(k, v)| format!("({}, {})", write_safe_string(k), write_safe_string(v)))
            .collect::<Vec<_>>()
            .join(", "))
}

fn write_inline_math_type(math_type: &MathType) -> String {
    match math_type {
        MathType::InlineMath => "InlineMath".to_string(),
        MathType::DisplayMath => "DisplayMath".to_string(),
    }
}

fn write_native_quote_type(quote_type: &QuoteType) -> String {
    match quote_type {
        QuoteType::SingleQuote => "SingleQuote".to_string(),
        QuoteType::DoubleQuote => "DoubleQuote".to_string(),
    }
}

fn write_inlines(inlines: &[Inline]) -> String {
    "[".to_string() + &(inlines.iter().map(write_inline).collect::<Vec<_>>().join(", ")) + "]"
}
fn write_citation_mode(mode: &CitationMode) -> String {
    match mode {
        CitationMode::NormalCitation => "NormalCitation".to_string(),
        CitationMode::SuppressAuthor => "SuppressAuthor".to_string(),
        CitationMode::AuthorInText => "AuthorInText".to_string(),
    }
}

fn write_inline(text: &Inline) -> String {
    match text {
        Inline::Math(math_struct) => {
            format!("Math {} {}", write_inline_math_type(&math_struct.math_type), write_safe_string(&math_struct.text))
        }
        Inline::Space(_) => "Space".to_string(),
        Inline::SoftBreak(_) => "SoftBreak".to_string(),
        Inline::LineBreak(_) => "LineBreak".to_string(),
        Inline::Str(str_struct) => format!("Str {}", write_safe_string(&str_struct.text)),
        Inline::Emph(emph_struct) => {
            let content_str = emph_struct.content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("Emph [{}]", content_str)
        },
        Inline::Underline(underline_struct) => {
            let content_str = underline_struct.content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("Underline [{}]", content_str)
        },
        Inline::SmallCaps(smallcaps_struct) => {
            let content_str = smallcaps_struct.content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("SmallCaps [{}]", content_str)
        },
        Inline::Superscript(superscript_struct) => {
            let content_str = superscript_struct.content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("Superscript [{}]", content_str)
        },
        Inline::Strong(strong_struct) => {
            let content_str = strong_struct.content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("Strong [{}]", content_str)
        },
        Inline::Span(span_struct) => {
            let content_str = span_struct.content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("Span {} [{}]", write_native_attr(&span_struct.attr), content_str)
        },
        Inline::Link(link_struct) => {
            let (url, title) = &link_struct.target;
            let content_str = link_struct.content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("Link {} [{}] ({} , {})", write_native_attr(&link_struct.attr), content_str, write_safe_string(url), write_safe_string(title))
        }
        Inline::Code(code_struct) => {
            format!("Code {} {}", write_native_attr(&code_struct.attr), write_safe_string(&code_struct.text))
        }
        Inline::RawInline(raw_struct) => {
            format!("RawInline (Format {}) {}", write_safe_string(&raw_struct.format), write_safe_string(&raw_struct.text))
        }
        Inline::Quoted(quoted_struct) => {
            let content_str = quoted_struct.content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("Quoted {} [{}]", write_native_quote_type(&quoted_struct.quote_type), content_str)
        }
        Inline::Note(note_struct) => {
            let content_str = note_struct.content.iter().map(write_block).collect::<Vec<_>>().join(", ");
            format!("Note [{}]", content_str)
        }
        Inline::Image(image_struct) => {
            let (url, title) = &image_struct.target;
            let content_str = image_struct.content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("Image {} [{}] ({} , {})", write_native_attr(&image_struct.attr), content_str, write_safe_string(url), write_safe_string(title))
        }
        Inline::Subscript(subscript_struct) => {
            let content_str = subscript_struct.content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("Subscript [{}]", content_str)
        }
        Inline::Strikeout(strikeout_struct) => {
            let content_str = strikeout_struct.content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("Strikeout [{}]", content_str)
        }
        Inline::Cite(cite_struct) => {
            format!("Cite [{}] {}", 
                cite_struct.citations.iter().map(|Citation { mode, note_num, hash, id, prefix, suffix }| {
                    format!("Citation {{ citationId = {}, citationPrefix = {}, citationSuffix = {}, citationMode = {}, citationNoteNum = {}, citationHash = {} }}",
                        write_safe_string(id), 
                        write_inlines(prefix), 
                        write_inlines(suffix),
                        write_citation_mode(mode),
                        note_num, hash)
                }).collect::<Vec<_>>().join(", "),
                write_inlines(&cite_struct.content))
        }
        _ => panic!("Unsupported inline type: {:?}", text),
    }
}

fn write_block(block: &Block) -> String {
    match block {
        Block::Paragraph { content } => {
            let content_str = content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("Para [{}]", content_str)
        }
        // Block::Header { level, attr, content } => {
        //     let content_str = content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
        //     format!("Header {} {} [{}]", level, write_native_attr(attr), content_str)
        // }
        // Block::CodeBlock { attr, text } => {
        //     format!("CodeBlock {} {}", write_native_attr(attr), write_safe_string(text))
        // }
        // Block::Quote { attr, blocks } => {
        //     let blocks_str = blocks.iter().map(write_block).collect::<Vec<_>>().join(", ");
        //     format!("Quote {} [{}]", write_native_attr(attr), blocks_str)
        // }
        // Block::List { attr, items } => {
        //     let items_str = items.iter().map(write_block).collect::<Vec<_>>().join(", ");
        //     format!("List {} [{}]", write_native_attr(attr), items_str)
        // }
        // Block::Table { attr, caption, headers, rows } => {
        //     let caption_str = caption.as_ref().map_or("None".to_string(), |c| write_safe_string(c));
        //     let headers_str = headers.iter().map(|h| write_safe_string(h)).collect::<Vec<_>>().join(", ");
        //     let rows_str = rows.iter().map(|row| row.iter().map(write_inline).collect::<Vec<_>>().join(", ")).collect::<Vec<_>>().join("; ");
        //     format!("Table {} (Caption: {}, Headers: [{}], Rows: [{}])", write_native_attr(attr), caption_str, headers_str, rows_str)
        // }
        // Block::HorizontalRule(_) => "HorizontalRule".to_string(),
        // Block::Div { attr, blocks } => {
        //     let blocks_str = blocks.iter().map(write_block).collect::<Vec<_>>().join(", ");
        //     format!("Div {} [{}]", write_native_attr(attr), blocks_str)
        // }
        _ => panic!("Unsupported block type: {:?}", block),
    }
}

pub fn write(pandoc: &Pandoc) -> String {
    String::from("[ ") + &pandoc.blocks.iter().map(write_block).collect::<Vec<_>>().join(" ") + " ]"
}