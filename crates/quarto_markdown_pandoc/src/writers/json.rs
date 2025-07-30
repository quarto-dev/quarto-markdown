/*
 * json.rs
 * Copyright (c) 2025 Posit, PBC
 */

use crate::pandoc::{Attr, Block, Inline, Inlines, Pandoc};
use crate::utils::autoid;
use serde_json::{Value, json};

fn write_attr(attr: &Attr) -> Value {
    json!([
        attr.0, // id
        attr.1, // classes
        attr.2
            .iter()
            .map(|(k, v)| json!([k, v]))
            .collect::<Vec<_>>()  // key-value pairs
    ])
}

fn write_inline(inline: &Inline) -> Value {
    match inline {
        Inline::Str(s) => json!({
            "t": "Str",
            "c": s.text
        }),
        Inline::Space(_) => json!({
            "t": "Space"
        }),
        Inline::LineBreak(_) => json!({
            "t": "LineBreak"
        }),
        Inline::SoftBreak(_) => json!({
            "t": "SoftBreak"
        }),
        Inline::Emph(e) => json!({
            "t": "Emph",
            "c": write_inlines(&e.content)
        }),
        Inline::Strong(s) => json!({
            "t": "Strong",
            "c": write_inlines(&s.content)
        }),
        Inline::Code(c) => json!({
            "t": "Code",
            "c": [write_attr(&c.attr), c.text]
        }),
        Inline::Math(m) => {
            let math_type = match m.math_type {
                crate::pandoc::MathType::InlineMath => json!({"t": "InlineMath"}),
                crate::pandoc::MathType::DisplayMath => json!({"t": "DisplayMath"}),
            };
            json!({
                "t": "Math",
                "c": [math_type, m.text]
            })
        },
        _ => json!({
            "t": "Str",
            "c": "[unimplemented]"
        }),
    }
}

fn write_inlines(inlines: &Inlines) -> Value {
    json!(inlines.iter().map(write_inline).collect::<Vec<_>>())
}

fn write_block(block: &Block) -> Value {
    match block {
        Block::Paragraph(para) => json!({
            "t": "Para",
            "c": write_inlines(&para.content)
        }),
        Block::Header(header) => {
            let mut attr = header.attr.clone();
            if attr.0.is_empty() {
                attr.0 = autoid::auto_generated_id(&header.content);
            }
            json!({
                "t": "Header",
                "c": [header.level, write_attr(&attr), write_inlines(&header.content)]
            })
        },
        Block::CodeBlock(codeblock) => json!({
            "t": "CodeBlock",
            "c": [write_attr(&codeblock.attr), codeblock.text]
        }),
        Block::Plain(plain) => json!({
            "t": "Plain",
            "c": write_inlines(&plain.content)
        }),
        Block::BulletList(bulletlist) => json!({
            "t": "BulletList",
            "c": bulletlist.content.iter().map(|blocks| blocks.iter().map(write_block).collect::<Vec<_>>()).collect::<Vec<_>>()
        }),
        _ => json!({
            "t": "Para",
            "c": [{"t": "Str", "c": "[unimplemented block]"}]
        }),
    }
}

fn write_pandoc(pandoc: &Pandoc) -> Value {
    json!({
        "pandoc-api-version": [1, 23, 1],
        "meta": {},
        "blocks": pandoc.blocks.iter().map(write_block).collect::<Vec<_>>(),
    })
}

pub fn write(pandoc: &Pandoc) -> String {
    write_pandoc(pandoc).to_string()
}
