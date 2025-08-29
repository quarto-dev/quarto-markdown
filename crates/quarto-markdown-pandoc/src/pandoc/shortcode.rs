/*
 * shortcode.rs
 * Copyright (c) 2025 Posit, PBC
 */

use crate::pandoc::inline::{Inline, Inlines, Span};
use std::collections::HashMap;

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

fn shortcode_value_span(str: String) -> Inline {
    let mut attr_hash = HashMap::new();
    attr_hash.insert("data-raw".to_string(), str.clone());
    attr_hash.insert("data-value".to_string(), str);
    attr_hash.insert("data-is-shortcode".to_string(), "1".to_string());

    Inline::Span(Span {
        attr: (
            "".to_string(),
            vec!["quarto-shortcode__-param".to_string()],
            attr_hash,
        ),
        content: vec![],
    })
}

fn shortcode_key_value_span(key: String, value: String) -> Inline {
    let mut attr_hash = HashMap::new();

    // this needs to be fixed and needs to use the actual source. We'll do that when we have source mapping
    attr_hash.insert(
        "data-raw".to_string(),
        format!("{} = {}", key.clone(), value.clone()),
    );
    attr_hash.insert("data-key".to_string(), key);
    attr_hash.insert("data-value".to_string(), value);
    attr_hash.insert("data-is-shortcode".to_string(), "1".to_string());

    Inline::Span(Span {
        attr: (
            "".to_string(),
            vec!["quarto-shortcode__-param".to_string()],
            attr_hash,
        ),
        content: vec![],
    })
}

pub fn shortcode_to_span(shortcode: Shortcode) -> Span {
    let mut attr_hash: HashMap<String, String> = HashMap::new();
    let mut content: Inlines = vec![shortcode_value_span(shortcode.name)];
    for arg in shortcode.positional_args {
        match arg {
            ShortcodeArg::String(text) => {
                content.push(shortcode_value_span(text));
            }
            ShortcodeArg::Number(num) => {
                content.push(shortcode_value_span(num.to_string()));
            }
            ShortcodeArg::Boolean(b) => {
                content.push(shortcode_value_span(if b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }));
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
                            content.push(shortcode_key_value_span(
                                key,
                                if b {
                                    "true".to_string()
                                } else {
                                    "false".to_string()
                                },
                            ));
                        }
                        ShortcodeArg::Shortcode(_) => {
                            eprintln!("PANIC - Quarto doesn't support nested shortcodes");
                            std::process::exit(1);
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
        attr: (
            "".to_string(),
            vec!["quarto-shortcode__".to_string()],
            attr_hash,
        ),
        content,
    }
}
