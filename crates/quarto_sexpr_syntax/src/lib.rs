#[macro_use]
mod generated;
mod syntax_node;

pub use self::generated::*;
pub use biome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use syntax_node::*;

use biome_rowan::RawSyntaxKind;

impl From<u16> for SexprSyntaxKind {
    fn from(d: u16) -> SexprSyntaxKind {
        assert!(d <= (SexprSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, SexprSyntaxKind>(d) }
    }
}

impl From<SexprSyntaxKind> for u16 {
    fn from(k: SexprSyntaxKind) -> u16 {
        k as u16
    }
}

impl biome_rowan::SyntaxKind for SexprSyntaxKind {
    const TOMBSTONE: Self = SexprSyntaxKind::TOMBSTONE;
    const EOF: Self = SexprSyntaxKind::EOF;

    fn is_bogus(&self) -> bool {
        match self {
            SexprSyntaxKind::SEXPR_BOGUS
            | SexprSyntaxKind::SEXPR_BOGUS_VALUE => true,
            _ => false
        }
    }

    fn to_bogus(&self) -> Self {
        match self {
            SexprSyntaxKind::SEXPR_SYMBOL_VALUE => SexprSyntaxKind::SEXPR_BOGUS_VALUE,
            _ => SexprSyntaxKind::SEXPR_BOGUS,
        }
    }

    #[inline]
    fn to_raw(&self) -> RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    #[inline]
    fn from_raw(raw: RawSyntaxKind) -> Self {
        Self::from(raw.0)
    }

    fn is_root(&self) -> bool {
        matches!(self, SexprSyntaxKind::SEXPR_ROOT)
    }

    fn is_list(&self) -> bool {
        SexprSyntaxKind::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(
            self,
            SexprSyntaxKind::NEWLINE | SexprSyntaxKind::WHITESPACE | SexprSyntaxKind::COMMENT
        )
    }

    fn to_string(&self) -> Option<&'static str> {
        SexprSyntaxKind::to_string(self)
    }
}
