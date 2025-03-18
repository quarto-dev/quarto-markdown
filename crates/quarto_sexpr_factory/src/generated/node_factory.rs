//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use biome_rowan::AstNode;
use quarto_sexpr_syntax::{
    SexprSyntaxElement as SyntaxElement, SexprSyntaxNode as SyntaxNode,
    SexprSyntaxToken as SyntaxToken, *,
};
pub fn sexpr_list(items: SexprItemList) -> SexprList {
    SexprList::unwrap_cast(SyntaxNode::new_detached(
        SexprSyntaxKind::SEXPR_LIST,
        [Some(SyntaxElement::Node(items.into_syntax()))],
    ))
}
pub fn sexpr_list_item(item: AnySexprValue) -> SexprListItem {
    SexprListItem::unwrap_cast(SyntaxNode::new_detached(
        SexprSyntaxKind::SEXPR_LIST_ITEM,
        [Some(SyntaxElement::Node(item.into_syntax()))],
    ))
}
pub fn sexpr_list_value(
    l_paren_token: SyntaxToken,
    sexpr_list: SexprList,
    r_paren_token: SyntaxToken,
) -> SexprListValue {
    SexprListValue::unwrap_cast(SyntaxNode::new_detached(
        SexprSyntaxKind::SEXPR_LIST_VALUE,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(sexpr_list.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn sexpr_root(value: AnySexprValue, eof_token: SyntaxToken) -> SexprRoot {
    SexprRoot::unwrap_cast(SyntaxNode::new_detached(
        SexprSyntaxKind::SEXPR_ROOT,
        [
            Some(SyntaxElement::Node(value.into_syntax())),
            Some(SyntaxElement::Token(eof_token)),
        ],
    ))
}
pub fn sexpr_symbol_value(value_token: SyntaxToken) -> SexprSymbolValue {
    SexprSymbolValue::unwrap_cast(SyntaxNode::new_detached(
        SexprSyntaxKind::SEXPR_SYMBOL_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn sexpr_item_list<I>(items: I) -> SexprItemList
where
    I: IntoIterator<Item = SexprListItem>,
    I::IntoIter: ExactSizeIterator,
{
    SexprItemList::unwrap_cast(SyntaxNode::new_detached(
        SexprSyntaxKind::SEXPR_ITEM_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn sexpr_bogus_value<I>(slots: I) -> SexprBogusValue
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    SexprBogusValue::unwrap_cast(SyntaxNode::new_detached(
        SexprSyntaxKind::SEXPR_BOGUS_VALUE,
        slots,
    ))
}
