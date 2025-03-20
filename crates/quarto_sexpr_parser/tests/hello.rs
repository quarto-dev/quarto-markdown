use quarto_sexpr_parser::*;  // Import your library

fn test_str_parse(input: &str) {
    let result = parse(input, SexprParserOptions { });
    println!("{:?}", result);
}

#[test]
fn test_single_symbol_parse() {
    test_str_parse("add");
}

#[test]
fn test_empty_list_parse() {
    test_str_parse("()");
}

#[test]
fn test_simple_list_parse() {
    test_str_parse("(add)");
}