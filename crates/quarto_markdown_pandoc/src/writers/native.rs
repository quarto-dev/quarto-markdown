
use crate::pandoc::{Pandoc, Block, Inline, MathType, Attr};

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

fn write_inline(text: &Inline) -> String {
    match text {
        Inline::Math { math_type, text } => {
            format!("Math {} {}", write_inline_math_type(math_type), write_safe_string(text))
        }
        Inline::Space => "Space".to_string(),
        Inline::SoftBreak => "SoftBreak".to_string(),
        Inline::Str { text } => format!("Str {}", write_safe_string(&text)),
        Inline::Emph { content } => {
            let content_str = content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("Emph [{}]", content_str)
        },
        Inline::Strong { content } => {
            let content_str = content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("Strong [{}]", content_str)
        },
        Inline::Span { attr, content } => {
            let content_str = content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("Span {} [{}]", write_native_attr(attr), content_str)
        },
        Inline::Link { attr, content, target } => {
            let (url, title) = target;
            let content_str = content.iter().map(write_inline).collect::<Vec<_>>().join(", ");
            format!("Link {} [{}] ({} , {})", write_native_attr(attr), content_str, write_safe_string(url), write_safe_string(title))
        }
        Inline::Code { text, attr } => {
            format!("Code {} {}", write_native_attr(attr), write_safe_string(text))
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