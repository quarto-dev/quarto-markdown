/*
 * json.rs
 * Copyright (c) 2025 Posit, PBC
 */

use crate::pandoc::{Attr, Block, Citation, CitationMode, Inline, Inlines, Pandoc};
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

fn write_citation_mode(mode: &CitationMode) -> Value {
    match mode {
        CitationMode::NormalCitation => json!({"t": "NormalCitation"}),
        CitationMode::AuthorInText => json!({"t": "AuthorInText"}),
        CitationMode::SuppressAuthor => json!({"t": "SuppressAuthor"}),
    }
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
        }
        Inline::Underline(u) => json!({
            "t": "Underline",
            "c": write_inlines(&u.content)
        }),
        Inline::Strikeout(s) => json!({
            "t": "Strikeout",
            "c": write_inlines(&s.content)
        }),
        Inline::Superscript(s) => json!({
            "t": "Superscript",
            "c": write_inlines(&s.content)
        }),
        Inline::Subscript(s) => json!({
            "t": "Subscript",
            "c": write_inlines(&s.content)
        }),
        Inline::SmallCaps(s) => json!({
            "t": "SmallCaps",
            "c": write_inlines(&s.content)
        }),
        Inline::Quoted(q) => {
            let quote_type = match q.quote_type {
                crate::pandoc::QuoteType::SingleQuote => json!({"t": "SingleQuote"}),
                crate::pandoc::QuoteType::DoubleQuote => json!({"t": "DoubleQuote"}),
            };
            json!({
                "t": "Quoted",
                "c": [quote_type, write_inlines(&q.content)]
            })
        }
        Inline::Link(link) => json!({
            "t": "Link",
            "c": [write_attr(&link.attr), write_inlines(&link.content), [link.target.0, link.target.1]]
        }),
        Inline::RawInline(raw) => json!({
            "t": "RawInline",
            "c": [raw.format.clone(), raw.text.clone()]
        }),
        Inline::Image(image) => json!({
            "t": "Image",
            "c": [write_attr(&image.attr), write_inlines(&image.content), [image.target.0, image.target.1]]
        }),
        Inline::Span(span) => json!({
            "t": "Span",
            "c": [write_attr(&span.attr), write_inlines(&span.content)]
        }),
        Inline::Note(note) => json!({
            "t": "Note",
            "c": write_blocks(&note.content)
        }),
        // we can't test this just yet because
        // our citationNoteNum counter doesn't match Pandoc's
        Inline::Cite(cite) => json!({
            "t": "Cite",
            "c": cite.citations.iter().map(|citation| {
                json!({
                    "citationId": citation.id.clone(),
                    "citationPrefix": write_inlines(&citation.prefix),
                    "citationSuffix": write_inlines(&citation.suffix),
                    "citationMode": write_citation_mode(&citation.mode),
                    "citationHash": citation.hash,
                    "citationNoteNum": citation.note_num
                })
            }).collect::<Vec<_>>()
        }),
        Inline::Shortcode(_) | Inline::NoteReference(_) => {
            panic!("Unsupported inline type: {:?}", inline)
        }
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
        }
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

fn write_blocks(blocks: &[Block]) -> Value {
    json!(blocks.iter().map(write_block).collect::<Vec<_>>())
}

fn write_pandoc(pandoc: &Pandoc) -> Value {
    json!({
        "pandoc-api-version": [1, 23, 1],
        "meta": {},
        "blocks": write_blocks(&pandoc.blocks),
    })
}

pub fn write(pandoc: &Pandoc) -> String {
    write_pandoc(pandoc).to_string()
}
