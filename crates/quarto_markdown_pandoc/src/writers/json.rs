/*
 * json.rs
 * Copyright (c) 2025 Posit, PBC
 */

use crate::pandoc::{Pandoc, Block, Inline, MathType, Attr};
use serde_json::{json, Value};

fn write_json(pandoc: &Pandoc) -> Value {
    json!({
        "pandoc-api-version": [1, 23, 1],
        "meta": {},
        "blocks": [],
    })
}

pub fn write(pandoc: &Pandoc) -> String {
    write_json(pandoc).to_string()
}