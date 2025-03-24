//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::all)]
#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SexprSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    L_PAREN,
    R_PAREN,
    NIL_KW,
    SEXPR_SYMBOL_LITERAL,
    ERROR_TOKEN,
    NEWLINE,
    WHITESPACE,
    IDENT,
    COMMENT,
    SEXPR_ROOT,
    SEXPR_LIST_VALUE,
    SEXPR_SYMBOL_VALUE,
    SEXPR_ITEM_LIST,
    SEXPR_LIST,
    SEXPR_BOGUS,
    SEXPR_BOGUS_VALUE,
    #[doc(hidden)]
    __LAST,
}
use self::SexprSyntaxKind::*;
impl SexprSyntaxKind {
    pub const fn is_punct(self) -> bool {
        match self {
            L_PAREN | R_PAREN => true,
            _ => false,
        }
    }
    pub const fn is_literal(self) -> bool {
        match self {
            SEXPR_SYMBOL_LITERAL => true,
            _ => false,
        }
    }
    pub const fn is_list(self) -> bool {
        match self {
            SEXPR_ITEM_LIST | SEXPR_LIST => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<SexprSyntaxKind> {
        let kw = match ident {
            "nil" => NIL_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            L_PAREN => "(",
            R_PAREN => ")",
            SEXPR_SYMBOL_LITERAL => "symbol literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { ['('] => { $ crate :: SexprSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: SexprSyntaxKind :: R_PAREN } ; [nil] => { $ crate :: SexprSyntaxKind :: NIL_KW } ; [ident] => { $ crate :: SexprSyntaxKind :: IDENT } ; [EOF] => { $ crate :: SexprSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: SexprSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: SexprSyntaxKind :: HASH } ; }
