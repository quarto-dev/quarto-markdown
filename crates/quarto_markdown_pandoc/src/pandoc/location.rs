/*
 * location.rs
 * Copyright (c) 2025 Posit, PBC
 */

////////////////////////////////////////////////////////////////////////////////////////////////////
// Source location tracking

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    pub offset: usize,
    pub row: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    pub start: Location,
    pub end: Location,
}

pub trait SourceLocation {
    fn filename(&self) -> Option<String>;
    fn range(&self) -> Range;
}

pub fn node_location(node: &tree_sitter::Node) -> Range {
    let start = node.start_position();
    let end = node.end_position();
    Range {
        start: Location {
            offset: node.start_byte(),
            row: start.row,
            column: start.column,
        },
        end: Location {
            offset: node.end_byte(),
            row: end.row,
            column: end.column,
        },
    }
}

pub fn empty_range() -> Range {
    Range {
        start: Location {
            offset: 0,
            row: 0,
            column: 0,
        },
        end: Location {
            offset: 0,
            row: 0,
            column: 0,
        },
    }
}

#[macro_export]
macro_rules! impl_source_location {
    ($($type:ty),*) => {
        $(
            impl SourceLocation for $type {
                fn filename(&self) -> Option<String> {
                    self.filename.clone()
                }

                fn range(&self) -> Range {
                    self.range.clone()
                }
            }
        )*
    };
}
