
/*
 * native.rs
 * Copyright (c) 2025 Posit, PBC
 */

use crate::pandoc::{Attr, Block, Inline, MathType, Pandoc, QuoteType};

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
        Inline::Quoted(quoted_struct) => {
            let content_str = quoted_struct.content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("Quoted {} [{}]", write_native_quote_type(&quoted_struct.quote_type), content_str)
        }
        _ => panic!("Unsupported inline type: {:?}", text),
    }
}

pub fn write(pandoc: &Pandoc) -> String {
    String::from("[ ") + &pandoc.blocks.iter().map(|block| {
        match block {
            Block::Paragraph { content } => String::from("Para [ ") +
                &(content
                    .iter().map(write_inline)
                    .collect::<Vec<_>>().join(", ")) + " ]",
            _ => panic!("Expected Paragraph block, got {:?}", block),
        }
    }).collect::<Vec<String>>().join(" ") + " ]"
}