use crate::errors;
use crate::errors::parse_is_good;
use crate::filters::FilterReturn::Unchanged;
use crate::filters::topdown_traverse;
use crate::filters::{Filter, FilterReturn};
use crate::pandoc::block::MetaBlock;
use crate::pandoc::meta::parse_metadata_strings;
use crate::pandoc::{self, Block, Meta};
use crate::pandoc::{MetaValue, rawblock_to_meta};
use crate::traversals;
use std::io::Write;
// use tree_sitter::LogType;
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
    let mut error_messages: Vec<String> = Vec::new();
    // let mut found_error: bool = false;

    // parser
    //     .parser
    //     .set_logger(Some(Box::new(|log_type, message| match log_type {
    //         LogType::Parse => {
    //             // if message.contains("detect_error") {
    //             //     found_error = true;
    //             // }
    //             eprintln!("tree-sitter: {:?}, {}", log_type, message);
    //         }
    //         _ => {}
    //     })));

    let tree = parser
        .parse(&input_bytes, None)
        .expect("Failed to parse input");

    let depth = crate::utils::concrete_tree_depth::concrete_tree_depth(&tree);
    // this is here mostly to prevent our fuzzer from blowing the stack
    // with a deeply nested document
    if depth > 100 {
        error_messages.push(format!(
            "The input document is too deeply nested (max depth: {} > 100).",
            depth
        ));
        return Err(error_messages);
    }

    let errors = parse_is_good(&tree);
    print_whole_tree(&mut tree.walk(), &mut output_stream);
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

    let mut result = pandoc::treesitter_to_pandoc(&mut output_stream, &tree, &input_bytes)?;
    let mut meta_from_parses = Meta::default();

    result = {
        let mut filter = Filter::new().with_raw_block(|rb| {
            if rb.format != "quarto_minus_metadata" {
                return Unchanged(rb);
            }
            let filename = rb.filename.clone();
            let range = rb.range.clone();
            let result = rawblock_to_meta(rb);
            let is_lexical = {
                let val = result.get("_scope");
                matches!(val, Some(MetaValue::MetaString(s)) if s == "lexical")
            };

            if is_lexical {
                let mut inner_meta_from_parses = Meta::default();
                let mut meta_map = match parse_metadata_strings(
                    MetaValue::MetaMap(result),
                    &mut inner_meta_from_parses,
                ) {
                    MetaValue::MetaMap(m) => m,
                    _ => panic!("Expected MetaMap from parse_metadata_strings"),
                };
                for (k, v) in inner_meta_from_parses {
                    meta_map.insert(k, v);
                }
                return FilterReturn::FilterResult(
                    vec![Block::BlockMetadata(MetaBlock {
                        meta: meta_map,
                        filename,
                        range,
                    })],
                    false,
                );
            } else {
                let meta_map =
                    match parse_metadata_strings(MetaValue::MetaMap(result), &mut meta_from_parses)
                    {
                        MetaValue::MetaMap(m) => m,
                        _ => panic!("Expected MetaMap from parse_metadata_strings"),
                    };
                for (k, v) in meta_map {
                    meta_from_parses.insert(k, v);
                }
                return FilterReturn::FilterResult(vec![], false);
            }
        });
        topdown_traverse(result, &mut filter)
    };
    for (k, v) in meta_from_parses.into_iter() {
        result.meta.insert(k, v);
    }
    Ok(result)
}
