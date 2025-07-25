/*
 * traversals.rs
 * 
 * Copyright (c) 2025 Posit, PBC
 * 
 * traversal helpers for tree-sitter-qmd's MarkdownCursor.
 * 
 * We can't use tree-sitter walking APIs directly because it involves
 * a mix of block and inline parsers that are handled by
 * two separate tree-sitter grammars.
 * 
 */

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum TraversePhase {
    Enter,
    Exit,
}

pub fn topdown_traverse_concrete_tree<F>(
    cursor: &mut tree_sitter_qmd::MarkdownCursor,
    visitor: &mut F,
) where
    F: for<'a> FnMut(&'a tree_sitter::Node, TraversePhase) -> bool,
{
    let node = cursor.node();
    if visitor(&node, TraversePhase::Enter) {
        if cursor.goto_first_child() {
            topdown_traverse_concrete_tree(cursor, visitor);
            loop {
                if !cursor.goto_next_sibling() {
                    break;
                }
                topdown_traverse_concrete_tree(cursor, visitor);
            }
            cursor.goto_parent();
        }
    }
        
    visitor(&node, TraversePhase::Exit);
}

pub fn bottomup_traverse_concrete_tree<F, T>(
    cursor: &mut tree_sitter_qmd::MarkdownCursor,
    visitor: &mut F,
    input_bytes: &[u8],
) -> (String, T) where
    F: for<'a> FnMut(&'a tree_sitter::Node, Vec<(String, T)>, &[u8]) -> T,
{
    let node = cursor.node();
    let mut children = Vec::new();
    
    if cursor.goto_first_child() {
        loop {
            children.push(bottomup_traverse_concrete_tree(cursor, visitor, input_bytes));
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
    
    (node.kind().to_string(), visitor(&node, children, input_bytes))
}