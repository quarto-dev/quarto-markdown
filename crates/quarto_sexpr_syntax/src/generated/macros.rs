//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](biome_rowan::SyntaxNode::kind)"]
#[doc = r" of the provided [biome_rowan::SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
    ($ node : expr , $ pattern : pat => $ body : expr) => {
        match $node {
            node => match $crate::SexprSyntaxNode::kind(&node) {
                $crate::SexprSyntaxKind::SEXPR_LIST => {
                    let $pattern = unsafe { $crate::SexprList::new_unchecked(node) };
                    $body
                }
                $crate::SexprSyntaxKind::SEXPR_LIST_ITEM => {
                    let $pattern = unsafe { $crate::SexprListItem::new_unchecked(node) };
                    $body
                }
                $crate::SexprSyntaxKind::SEXPR_LIST_VALUE => {
                    let $pattern = unsafe { $crate::SexprListValue::new_unchecked(node) };
                    $body
                }
                $crate::SexprSyntaxKind::SEXPR_ROOT => {
                    let $pattern = unsafe { $crate::SexprRoot::new_unchecked(node) };
                    $body
                }
                $crate::SexprSyntaxKind::SEXPR_SYMBOL_VALUE => {
                    let $pattern = unsafe { $crate::SexprSymbolValue::new_unchecked(node) };
                    $body
                }
                $crate::SexprSyntaxKind::SEXPR_BOGUS => {
                    let $pattern = unsafe { $crate::SexprBogus::new_unchecked(node) };
                    $body
                }
                $crate::SexprSyntaxKind::SEXPR_BOGUS_VALUE => {
                    let $pattern = unsafe { $crate::SexprBogusValue::new_unchecked(node) };
                    $body
                }
                $crate::SexprSyntaxKind::SEXPR_ITEM_LIST => {
                    let $pattern = unsafe { $crate::SexprItemList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
