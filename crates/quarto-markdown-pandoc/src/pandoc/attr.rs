/*
 * attr.rs
 * Copyright (c) 2025 Posit, PBC
 */

use std::collections::HashMap;

pub fn empty_attr() -> Attr {
    ("".to_string(), vec![], HashMap::new())
}

pub type Attr = (String, Vec<String>, HashMap<String, String>);

pub fn is_empty_attr(attr: &Attr) -> bool {
    attr.0.is_empty() && attr.1.is_empty() && attr.2.is_empty()
}
