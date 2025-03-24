//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_rowan::{
    AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind,
};
use quarto_sexpr_syntax::{SexprSyntaxKind, SexprSyntaxKind::*, T, *};
#[derive(Debug)]
pub struct SexprSyntaxFactory;
impl SyntaxFactory for SexprSyntaxFactory {
    type Kind = SexprSyntaxKind;
    #[allow(unused_mut)]
    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            SEXPR_BOGUS | SEXPR_BOGUS_VALUE => {
                RawSyntaxNode::new(kind, children.into_iter().map(Some))
            }
            SEXPR_LIST_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T!['('] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if SexprList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![')'] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SEXPR_LIST_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SEXPR_LIST_VALUE, children)
            }
            SEXPR_ROOT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if AnySexprValue::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![EOF] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SEXPR_ROOT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SEXPR_ROOT, children)
            }
            SEXPR_SYMBOL_VALUE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == SEXPR_SYMBOL_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SEXPR_SYMBOL_VALUE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SEXPR_SYMBOL_VALUE, children)
            }
            SEXPR_LIST => Self::make_node_list_syntax(kind, children, AnySexprValue::can_cast),
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
