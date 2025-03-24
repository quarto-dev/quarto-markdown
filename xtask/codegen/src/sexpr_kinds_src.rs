use crate::kind_src::KindsSrc;

pub const SEXPR_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
    ],
    keywords: &["nil"],
    literals: &["SEXPR_SYMBOL_LITERAL"],

    tokens: &[
        "ERROR_TOKEN",
        "NEWLINE",
        "WHITESPACE",
        "IDENT",
        "COMMENT",
    ],
    nodes: &[
        "SEXPR_ROOT", 
        "SEXPR_LIST_VALUE", 
        "SEXPR_SYMBOL_VALUE",
        "SEXPR_ITEM_LIST",
        "SEXPR_LIST",
        // bogus nodes?
        "SEXPR_BOGUS",
        "SEXPR_BOGUS_VALUE",        
    ],
};