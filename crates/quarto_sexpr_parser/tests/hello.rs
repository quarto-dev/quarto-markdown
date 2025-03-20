use quarto_sexpr_parser::*;  // Import your library

#[test]
fn test_public_function() {
    let input = "add";
    let result = parse(&input, SexprParserOptions { });
    println!("{:?}", result);
}