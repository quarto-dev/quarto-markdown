
use crate::pandoc::{Pandoc, Block, Inline, MathType};

fn write_inline_math_type(math_type: &MathType) -> String {
    match math_type {
        MathType::InlineMath => "InlineMath".to_string(),
        MathType::DisplayMath => "DisplayMath".to_string(),
    }
}

fn write_safe_string(text: &str) -> String {
    format!("\"{}\"", text.replace("\\", "\\\\").replace("\"", "\\\""))
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