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
    GoToSiblings,
}

pub fn topdown_traverse_concrete_tree<F>(
    cursor: &mut tree_sitter_qmd::MarkdownCursor,
    visitor: &mut F,
) where
    F: for<'a> FnMut(&'a tree_sitter::Node, TraversePhase) -> bool,
{
    let mut stack = vec![TraversePhase::Enter];
    while stack.len() > 0 {
        let top = stack.pop().unwrap();
        match top {
            TraversePhase::Enter => {
                stack.push(TraversePhase::GoToSiblings);
                if visitor(&cursor.node(), TraversePhase::Enter) {
                    if cursor.goto_first_child() {
                        stack.push(TraversePhase::Enter);
                    }
                }
            }
            TraversePhase::GoToSiblings => {
                if cursor.goto_next_sibling() {
                    stack.push(TraversePhase::Enter);
                } else {
                    cursor.goto_parent();
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum BottomUpTraversePhase<'a, T: std::fmt::Debug> {
    Enter(tree_sitter::Node<'a>),
    GoToSiblings(tree_sitter::Node<'a>, Vec<(String, T)>), // accumulated children
    Exit(tree_sitter::Node<'a>),
}

pub fn bottomup_traverse_concrete_tree<F, T: std::fmt::Debug>(
    cursor: &mut tree_sitter_qmd::MarkdownCursor,
    visitor: &mut F,
    input_bytes: &[u8],
) -> (String, T)
where
    F: for<'a> FnMut(&'a tree_sitter::Node, Vec<(String, T)>, &[u8]) -> T,
{
    let mut stack: Vec<BottomUpTraversePhase<T>> =
        vec![BottomUpTraversePhase::Enter(cursor.node())];

    loop {
        let top = stack.pop().unwrap();
        match top {
            BottomUpTraversePhase::Enter(node) => {
                stack.push(BottomUpTraversePhase::GoToSiblings(node, Vec::new()));
                if cursor.goto_first_child() {
                    stack.push(BottomUpTraversePhase::Enter(cursor.node()));
                } else {
                    stack.push(BottomUpTraversePhase::Exit(node));
                }
            }
            BottomUpTraversePhase::GoToSiblings(node, vec) => {
                stack.push(BottomUpTraversePhase::GoToSiblings(node, vec));
                if cursor.goto_next_sibling() {
                    stack.push(BottomUpTraversePhase::Enter(cursor.node()));
                } else {
                    stack.push(BottomUpTraversePhase::Exit(node));
                    cursor.goto_parent();
                }
            }
            BottomUpTraversePhase::Exit(node) => {
                let Some(BottomUpTraversePhase::GoToSiblings(_, children)) = stack.pop() else {
                    panic!("Expected GoToSiblings phase on stack");
                };
                let (kind, result) = (
                    node.kind().to_string(),
                    visitor(&node, children, input_bytes),
                );
                match stack.last_mut() {
                    None => return (kind, result), // we are done
                    Some(BottomUpTraversePhase::GoToSiblings(_, next_children)) => {
                        next_children.push((kind, result));
                    }
                    _ => {
                        panic!("Expected GoToSiblings phase on stack");
                    }
                }
            }
        }
    }

    // let node = cursor.node();
    // let mut children = Vec::new();
    // // state 1
    // if cursor.goto_first_child() {
    //     loop {
    //         // state 2
    //         let result = bottomup_traverse_concrete_tree(cursor, visitor, input_bytes);
    //         children.push(result);
    //         if !cursor.goto_next_sibling() {
    //             break;
    //         }
    //     }
    //     cursor.goto_parent();
    // }
    // // state 3
    // (
    //     node.kind().to_string(),
    //     visitor(&node, children, input_bytes),
    // )
}
