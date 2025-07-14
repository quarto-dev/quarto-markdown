use std::io::{self, Read};
use tree_sitter_qmd::MarkdownParser;

mod errors;
use errors::parse_is_good;

fn print_whole_tree(cursor: &mut tree_sitter_qmd::MarkdownCursor, indent: usize) {
    println!("{}node: {:?}", "  ".repeat(indent), cursor.node());
    if cursor.goto_first_child() {
        loop {
            print_whole_tree(cursor, indent + 1);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut parser = MarkdownParser::default();
    let input_bytes = input.as_bytes();
    let tree = parser.parse(&input_bytes, None).expect("Failed to parse input");
    let errors = parse_is_good(&tree);
    if !errors.is_empty() {
        let mut cursor = tree.walk();
        // print_whole_tree(&mut cursor, 0);
        for error in errors {
            cursor.goto_id(error);
            eprintln!("{}", errors::error_message(&mut cursor, &input_bytes));
        }
        return;
    }
    let mut cursor = tree.walk();

    print_whole_tree(&mut cursor, 0);

    // println!("{:?}", cursor.node());
    // cursor.goto_first_child();
    // println!("{:?}", cursor.node());
    // cursor.goto_first_child();
    // println!("{:?}", cursor.node());
    // cursor.goto_first_child();
    // println!("{:?}", cursor.node());
    // cursor.goto_next_sibling();
    // println!("{:?}", cursor.node());
    // cursor.goto_first_child();
    // println!("{:?}", cursor.node());
    // cursor.goto_next_sibling();
    // println!("{:?}", cursor.node());
    // cursor.goto_next_sibling();
    // println!("{:?}", cursor.node());
    // cursor.goto_next_sibling();
    // println!("{:?}", cursor.node());
    // cursor.goto_next_sibling();
    // println!("{:?}", cursor.node());
    // cursor.goto_next_sibling();
    // println!("{:?}", cursor.node());
    // cursor.goto_next_sibling();
    // println!("{:?}", cursor.node());
    // cursor.goto_next_sibling();
    // println!("{:?}", cursor.node());
}
