use std::marker::PhantomData;

use quarto_markdown_syntax::MarkdownLanguage;
use quarto_markdown_syntax::MarkdownRoot;
use quarto_markdown_syntax::MarkdownSyntaxKind;
use quarto_markdown_syntax::MarkdownSyntaxNode;
use biome_parser::event::Event;
use biome_parser::prelude::ParseDiagnostic;
use biome_parser::prelude::Trivia;
use biome_parser::AnyParse;
use biome_rowan::AstNode;
use biome_rowan::NodeCache;
use biome_rowan::TextRange;
use biome_rowan::TextSize;
use biome_rowan::TriviaPieceKind;
use biome_unicode_table::Dispatch;
use tree_sitter::Tree;

use crate::treesitter::NodeTypeExt;
use crate::treesitter::Preorder;
use crate::treesitter::WalkEvent;
use crate::ParseError;
use crate::MarkdownLosslessTreeSink;
use crate::MarkdownParserOptions;

/// A utility struct for managing the result of a parser job
#[derive(Debug, Clone)]
pub struct Parse<T> {
    root: MarkdownSyntaxNode,
    errors: Vec<ParseError>,
    _ty: PhantomData<T>,
}

impl<T> Parse<T> {
    pub fn new(root: MarkdownSyntaxNode, errors: Vec<ParseError>) -> Parse<T> {
        Parse {
            root,
            errors,
            _ty: PhantomData,
        }
    }

    pub fn cast<N: AstNode<Language = MarkdownLanguage>>(self) -> Option<Parse<N>> {
        if N::can_cast(self.syntax().kind()) {
            Some(Parse::new(self.root, self.errors))
        } else {
            None
        }
    }

    /// The syntax node represented by this Parse result
    pub fn syntax(&self) -> MarkdownSyntaxNode {
        self.root.clone()
    }

    /// Get the errors which occurred when parsing
    pub fn errors(&self) -> &[ParseError] {
        self.errors.as_slice()
    }

    /// Get the errors which occurred when parsing
    pub fn into_errors(self) -> Vec<ParseError> {
        self.errors
    }

    /// Returns [true] if the parser encountered some errors during the parsing.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl<T: AstNode<Language = MarkdownLanguage>> Parse<T> {
    /// Convert this parse result into a typed AST node.
    ///
    /// # Panics
    /// Panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> T {
        self.try_tree().unwrap_or_else(|| {
            panic!(
                "Expected tree to be a {} but root is:\n{:#?}",
                std::any::type_name::<T>(),
                self.syntax()
            )
        })
    }

    /// Try to convert this parse's untyped syntax node into an AST node.
    pub fn try_tree(&self) -> Option<T> {
        T::cast(self.syntax())
    }

    /// Convert this parse into a result
    pub fn into_result(self) -> Result<T, Vec<ParseError>> {
        if !self.has_errors() {
            Ok(self.tree())
        } else {
            Err(self.errors)
        }
    }
}

impl<T> From<Parse<T>> for AnyParse {
    fn from(parse: Parse<T>) -> Self {
        let root = parse.syntax();
        let errors = parse.into_errors();
        let diagnostics = errors
            .into_iter()
            .map(ParseError::into_diagnostic)
            .collect();
        Self::new(
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        )
    }
}

pub fn parse_text(
    text: &str,
    _options: MarkdownParserOptions,
) -> (Vec<Event<MarkdownSyntaxKind>>, Vec<Trivia>, Vec<ParseError>) {
    // let mut parser = tree_sitter::Parser::new();
    // parser
    //     .set_language(&tree_sitter_sexpr::LANGUAGE.into())
    //     .unwrap();
    // let ast = parser.parse(text, None).unwrap();

    if ast.root_node().has_error() {
        // TODO: In the long term we want an error resiliant parser.
        // This would probably only be able to happen if we swap out tree sitter
        // for a hand written recursive descent pratt parser using the Biome infra.
        return parse_failure();
    }

    parse_tree(ast, text)
}

// fn parse_tree(ast: Tree, text: &str) -> (Vec<Event<SexprSyntaxKind>>, Vec<Trivia>, Vec<ParseError>) {
fn parse_tree(ast: Tree, text: &str) -> None {
    // let mut walker = SexprWalk::new(text);

    // let root = ast.root_node();
    // let mut iter = root.preorder();
    // walker.walk(&mut iter);

    // walker.parse.drain();
}