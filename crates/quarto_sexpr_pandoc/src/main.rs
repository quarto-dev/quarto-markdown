use quarto_sexpr_parser::*;
use quarto_sexpr_syntax::*;
use std::io::{self, Read};

fn convert_to_pandoc_ast(node: &SexprSyntaxNode, source: &String) -> String {
    let mut preorder = node.preorder();
    let mut result = String::new();
    let mut is_first = vec![true];
    while let Some(event) = preorder.next() {
        match event {
            WalkEvent::Enter(node) => match node.kind() {
                SexprSyntaxKind::SEXPR_ROOT => {
                    result.push_str("{\"pandoc-api-version\":[1,23,1],\"meta\":{},\"blocks\":[{\"t\":\"Div\",\"c\":[[\"\",[],[]],[");
                }
                SexprSyntaxKind::SEXPR_LIST => {
                    if !is_first.last().unwrap() {
                        result.push_str(",");
                    }
                    let ix = is_first.len() - 1;
                    is_first[ix] = false;
                    is_first.push(true);
                    result.push_str("{\"t\":\"Div\",\"c\":[[\"\",[],[]],[");
                }
                SexprSyntaxKind::SEXPR_SYMBOL_VALUE => {
                    if !is_first.last().unwrap() {
                        result.push_str(",");
                    }
                    let ix = is_first.len() - 1;
                    is_first[ix] = false;
                    result.push_str("{\"t\":\"Para\",\"c\":[{\"t\":\"Str\",\"c\":\"");
                    let range = node.text_trimmed_range();
                    result.push_str(
                        source
                            .chars()
                            .skip(range.start().into())
                            .take(range.len().into())
                            .collect::<String>()
                            .as_str(),
                    );
                }
                _ => {}
            },
            WalkEvent::Leave(node) => match node.kind() {
                SexprSyntaxKind::SEXPR_ROOT => {
                    result.push_str("]]}]}");
                }
                SexprSyntaxKind::SEXPR_LIST => {
                    result.push_str("]]}");
                    is_first.pop();
                }
                SexprSyntaxKind::SEXPR_SYMBOL_VALUE => {
                    result.push_str("\"}]}");
                }
                _ => {}
            },
        }
    }
    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let result = parse(&input, SexprParserOptions {});
    let root = result.syntax();
    print!("{}", convert_to_pandoc_ast(&root, &input));
}
