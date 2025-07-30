/*
 * autoid.rs
 * Copyright (c) 2025 Posit, PBC
 */

use crate::pandoc::{Inline, Inlines};
use std::fmt::Write;

fn collect_text(inlines: &Inlines, result: &mut String) {
    for inline in inlines {
        match inline {
            Inline::Str(s) => {
                write!(result, "{}", s.text).unwrap();
            }
            Inline::Space(_) => {
                write!(result, " ").unwrap();
            }
            Inline::Emph(e) => {
                collect_text(&e.content, result);
            }
            Inline::Strong(s) => {
                collect_text(&s.content, result);
            }
            Inline::Code(c) => {
                write!(result, "{}", c.text).unwrap();
            }
            _ => {
                // Skip other inline types for ID generation
            }
        }
    }
}

pub fn auto_generated_id(inlines: &Inlines) -> String {
    let mut text = String::new();
    collect_text(inlines, &mut text);
    
    // Convert to lowercase and replace spaces/special chars with hyphens
    text
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}