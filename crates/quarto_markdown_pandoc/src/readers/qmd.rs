use crate::errors;
use crate::errors::parse_is_good;
use crate::pandoc;
use crate::traversals;
use std::io::Write;
use tree_sitter::LogType;
use tree_sitter_qmd::MarkdownParser;

fn print_whole_tree<T: Write>(cursor: &mut tree_sitter_qmd::MarkdownCursor, buf: &mut T) {
    let mut depth = 0;
    traversals::topdown_traverse_concrete_tree(cursor, &mut |node, phase| {
        if phase == traversals::TraversePhase::Enter {
            writeln!(buf, "{}{}: {:?}", "  ".repeat(depth), node.kind(), node).unwrap();
            depth += 1;
        } else {
            depth -= 1;
        }
        true // continue traversing
    });
}

pub fn read<T: Write>(
    input_bytes: &[u8],
    mut output_stream: &mut T,
) -> Result<pandoc::Pandoc, Vec<String>> {
    let mut parser = MarkdownParser::default();
    // let mut found_error: bool = false;

    // parser
    //     .parser
    //     .set_logger(Some(Box::new(|log_type, message| match log_type {
    //         LogType::Parse => {
    //             if message.contains("detect_error") {
    //                 found_error = true;
    //             }
    //             eprintln!("tree-sitter: {:?}, {}", log_type, message);
    //         }
    //         _ => {}
    //     })));

    let tree = parser
        .parse(&input_bytes, None)
        .expect("Failed to parse input");
    // println!("Found error: {}", found_error);
    let errors = parse_is_good(&tree);
    print_whole_tree(&mut tree.walk(), &mut output_stream);
    let mut error_messages: Vec<String> = Vec::new();
    if !errors.is_empty() {
        let mut cursor = tree.walk();
        for error in errors {
            cursor.goto_id(error);
            error_messages.push(errors::error_message(&mut cursor, &input_bytes));
        }
    }
    if !error_messages.is_empty() {
        return Err(error_messages);
    }

    let pandoc = pandoc::treesitter_to_pandoc(&mut output_stream, &tree, &input_bytes);
    Ok(pandoc)
}
