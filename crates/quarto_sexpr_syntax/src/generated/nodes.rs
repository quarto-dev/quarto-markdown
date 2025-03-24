//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
use crate::{
    macros::map_syntax_node,
    SexprLanguage as Language, SexprSyntaxElement as SyntaxElement,
    SexprSyntaxElementChildren as SyntaxElementChildren,
    SexprSyntaxKind::{self as SyntaxKind, *},
    SexprSyntaxList as SyntaxList, SexprSyntaxNode as SyntaxNode, SexprSyntaxToken as SyntaxToken,
};
use biome_rowan::{support, AstNode, RawSyntaxKind, SyntaxKindSet, SyntaxResult};
#[allow(unused)]
use biome_rowan::{
    AstNodeList, AstNodeListIterator, AstNodeSlotMap, AstSeparatedList,
    AstSeparatedListNodesIterator,
};
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
#[doc = r" Sentinel value indicating a missing element in a dynamic node, where"]
#[doc = r" the slots are not statically known."]
#[allow(dead_code)]
pub(crate) const SLOT_MAP_EMPTY_VALUE: u8 = u8::MAX;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SexprListValue {
    pub(crate) syntax: SyntaxNode,
}
impl SexprListValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SexprListValueFields {
        SexprListValueFields {
            l_paren_token: self.l_paren_token(),
            sexpr_list: self.sexpr_list(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn sexpr_list(&self) -> SexprList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SexprListValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SexprListValueFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub sexpr_list: SexprList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SexprRoot {
    pub(crate) syntax: SyntaxNode,
}
impl SexprRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SexprRootFields {
        SexprRootFields {
            value: self.value(),
            eof_token: self.eof_token(),
        }
    }
    pub fn value(&self) -> SyntaxResult<AnySexprValue> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for SexprRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SexprRootFields {
    pub value: SyntaxResult<AnySexprValue>,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SexprSymbolValue {
    pub(crate) syntax: SyntaxNode,
}
impl SexprSymbolValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SexprSymbolValueFields {
        SexprSymbolValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SexprSymbolValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SexprSymbolValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySexprValue {
    SexprBogusValue(SexprBogusValue),
    SexprListValue(SexprListValue),
    SexprSymbolValue(SexprSymbolValue),
}
impl AnySexprValue {
    pub fn as_sexpr_bogus_value(&self) -> Option<&SexprBogusValue> {
        match &self {
            AnySexprValue::SexprBogusValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sexpr_list_value(&self) -> Option<&SexprListValue> {
        match &self {
            AnySexprValue::SexprListValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sexpr_symbol_value(&self) -> Option<&SexprSymbolValue> {
        match &self {
            AnySexprValue::SexprSymbolValue(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for SexprListValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SEXPR_LIST_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SEXPR_LIST_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SexprListValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SexprListValue")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("sexpr_list", &self.sexpr_list())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<SexprListValue> for SyntaxNode {
    fn from(n: SexprListValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<SexprListValue> for SyntaxElement {
    fn from(n: SexprListValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for SexprRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SEXPR_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SEXPR_ROOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SexprRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SexprRoot")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
            .finish()
    }
}
impl From<SexprRoot> for SyntaxNode {
    fn from(n: SexprRoot) -> SyntaxNode {
        n.syntax
    }
}
impl From<SexprRoot> for SyntaxElement {
    fn from(n: SexprRoot) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for SexprSymbolValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SEXPR_SYMBOL_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SEXPR_SYMBOL_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SexprSymbolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SexprSymbolValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<SexprSymbolValue> for SyntaxNode {
    fn from(n: SexprSymbolValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<SexprSymbolValue> for SyntaxElement {
    fn from(n: SexprSymbolValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<SexprBogusValue> for AnySexprValue {
    fn from(node: SexprBogusValue) -> AnySexprValue {
        AnySexprValue::SexprBogusValue(node)
    }
}
impl From<SexprListValue> for AnySexprValue {
    fn from(node: SexprListValue) -> AnySexprValue {
        AnySexprValue::SexprListValue(node)
    }
}
impl From<SexprSymbolValue> for AnySexprValue {
    fn from(node: SexprSymbolValue) -> AnySexprValue {
        AnySexprValue::SexprSymbolValue(node)
    }
}
impl AstNode for AnySexprValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SexprBogusValue::KIND_SET
        .union(SexprListValue::KIND_SET)
        .union(SexprSymbolValue::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SEXPR_BOGUS_VALUE | SEXPR_LIST_VALUE | SEXPR_SYMBOL_VALUE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SEXPR_BOGUS_VALUE => AnySexprValue::SexprBogusValue(SexprBogusValue { syntax }),
            SEXPR_LIST_VALUE => AnySexprValue::SexprListValue(SexprListValue { syntax }),
            SEXPR_SYMBOL_VALUE => AnySexprValue::SexprSymbolValue(SexprSymbolValue { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnySexprValue::SexprBogusValue(it) => &it.syntax,
            AnySexprValue::SexprListValue(it) => &it.syntax,
            AnySexprValue::SexprSymbolValue(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnySexprValue::SexprBogusValue(it) => it.syntax,
            AnySexprValue::SexprListValue(it) => it.syntax,
            AnySexprValue::SexprSymbolValue(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySexprValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnySexprValue::SexprBogusValue(it) => std::fmt::Debug::fmt(it, f),
            AnySexprValue::SexprListValue(it) => std::fmt::Debug::fmt(it, f),
            AnySexprValue::SexprSymbolValue(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySexprValue> for SyntaxNode {
    fn from(n: AnySexprValue) -> SyntaxNode {
        match n {
            AnySexprValue::SexprBogusValue(it) => it.into(),
            AnySexprValue::SexprListValue(it) => it.into(),
            AnySexprValue::SexprSymbolValue(it) => it.into(),
        }
    }
}
impl From<AnySexprValue> for SyntaxElement {
    fn from(n: AnySexprValue) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnySexprValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SexprListValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SexprRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SexprSymbolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SexprBogus {
    syntax: SyntaxNode,
}
impl SexprBogus {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for SexprBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SEXPR_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SEXPR_BOGUS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SexprBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SexprBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<SexprBogus> for SyntaxNode {
    fn from(n: SexprBogus) -> SyntaxNode {
        n.syntax
    }
}
impl From<SexprBogus> for SyntaxElement {
    fn from(n: SexprBogus) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SexprBogusValue {
    syntax: SyntaxNode,
}
impl SexprBogusValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for SexprBogusValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SEXPR_BOGUS_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SEXPR_BOGUS_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for SexprBogusValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SexprBogusValue")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<SexprBogusValue> for SyntaxNode {
    fn from(n: SexprBogusValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<SexprBogusValue> for SyntaxElement {
    fn from(n: SexprBogusValue) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SexprList {
    syntax_list: SyntaxList,
}
impl SexprList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for SexprList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SEXPR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SEXPR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<SexprList> {
        if Self::can_cast(syntax.kind()) {
            Some(SexprList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for SexprList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for SexprList {
    type Language = Language;
    type Node = AnySexprValue;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SexprList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SexprList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &SexprList {
    type Item = AnySexprValue;
    type IntoIter = AstNodeListIterator<Language, AnySexprValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for SexprList {
    type Item = AnySexprValue;
    type IntoIter = AstNodeListIterator<Language, AnySexprValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone)]
pub struct DebugSyntaxElementChildren(pub SyntaxElementChildren);
impl Debug for DebugSyntaxElementChildren {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.clone().0.map(DebugSyntaxElement))
            .finish()
    }
}
struct DebugSyntaxElement(SyntaxElement);
impl Debug for DebugSyntaxElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            SyntaxElement::Node(node) => {
                map_syntax_node ! (node . clone () , node => std :: fmt :: Debug :: fmt (& node , f))
            }
            SyntaxElement::Token(token) => Debug::fmt(token, f),
        }
    }
}
