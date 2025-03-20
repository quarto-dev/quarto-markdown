//! This module defines the Concrete Syntax Tree used by Biome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{SexprRoot, SexprSyntaxKind};
use biome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SexprLanguage;

impl Language for SexprLanguage {
    type Kind = SexprSyntaxKind;
    type Root = SexprRoot;
}

pub type SexprSyntaxNode = biome_rowan::SyntaxNode<SexprLanguage>;
pub type SexprSyntaxToken = biome_rowan::SyntaxToken<SexprLanguage>;
pub type SexprSyntaxElement = biome_rowan::SyntaxElement<SexprLanguage>;
pub type SexprSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<SexprLanguage>;
pub type SexprSyntaxElementChildren = biome_rowan::SyntaxElementChildren<SexprLanguage>;
pub type SexprSyntaxList = biome_rowan::SyntaxList<SexprLanguage>;
