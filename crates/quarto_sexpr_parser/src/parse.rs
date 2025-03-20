use std::marker::PhantomData;

use quarto_sexpr_syntax::SexprLanguage;
use quarto_sexpr_syntax::SexprRoot;
use quarto_sexpr_syntax::SexprSyntaxKind;
use quarto_sexpr_syntax::SexprSyntaxNode;
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
use crate::SexprLosslessTreeSink;
use crate::SexprParserOptions;

/// A utility struct for managing the result of a parser job
#[derive(Debug, Clone)]
pub struct Parse<T> {
    root: SexprSyntaxNode,
    errors: Vec<ParseError>,
    _ty: PhantomData<T>,
}

impl<T> Parse<T> {
    pub fn new(root: SexprSyntaxNode, errors: Vec<ParseError>) -> Parse<T> {
        Parse {
            root,
            errors,
            _ty: PhantomData,
        }
    }

    pub fn cast<N: AstNode<Language = SexprLanguage>>(self) -> Option<Parse<N>> {
        if N::can_cast(self.syntax().kind()) {
            Some(Parse::new(self.root, self.errors))
        } else {
            None
        }
    }

    /// The syntax node represented by this Parse result
    pub fn syntax(&self) -> SexprSyntaxNode {
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

impl<T: AstNode<Language = SexprLanguage>> Parse<T> {
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

pub fn parse(text: &str, options: SexprParserOptions) -> Parse<SexprRoot> {
    let mut cache = NodeCache::default();
    parse_sexpr_with_cache(text, options, &mut cache)
}

pub fn parse_sexpr_with_cache(
    text: &str,
    options: SexprParserOptions,
    cache: &mut NodeCache,
) -> Parse<SexprRoot> {
    tracing::debug_span!("parse").in_scope(move || {
        let (events, tokens, errors) = parse_text(text, options);

        // We've determined that passing diagnostics through does nothing.
        // They go into the tree-sink but come right back out. We think they
        // are a holdover from rust-analyzer that can be removed now. The real
        // errors are in `errors`.
        let _diagnostics = vec![];

        let mut tree_sink = SexprLosslessTreeSink::with_cache(text, &tokens, cache);
        println!("Events: {:?}", events);
        biome_parser::event::process(&mut tree_sink, events, _diagnostics);
        let (green, _diagnostics) = tree_sink.finish();

        Parse::new(green, errors)
    })
}

pub fn parse_text(
    text: &str,
    _options: SexprParserOptions,
) -> (Vec<Event<SexprSyntaxKind>>, Vec<Trivia>, Vec<ParseError>) {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_sexpr::LANGUAGE.into())
        .unwrap();

    let ast = parser.parse(text, None).unwrap();

    if ast.root_node().has_error() {
        // TODO: In the long term we want an error resiliant parser.
        // This would probably only be able to happen if we swap out tree sitter
        // for a hand written recursive descent pratt parser using the Biome infra.
        return parse_failure();
    }

    parse_tree(ast, text)
}

fn parse_failure() -> (Vec<Event<SexprSyntaxKind>>, Vec<Trivia>, Vec<ParseError>) {
    // Must provide a root node on failures, otherwise `tree_sink.finish()` fails
    let events = vec![
        Event::Start {
            kind: SexprSyntaxKind::SEXPR_ROOT,
            forward_parent: None,
        },
        Event::Finish,
    ];

    // No trivia
    let trivia = vec![];

    // Generate a single diagnostic, wrap it in our error type
    let span: Option<TextRange> = None;
    let diagnostic = ParseDiagnostic::new("Failed to parse due to syntax errors.", span);
    let error = ParseError::from(diagnostic);
    let errors = vec![error];

    (events, trivia, errors)
}

fn parse_tree(ast: Tree, text: &str) -> (Vec<Event<SexprSyntaxKind>>, Vec<Trivia>, Vec<ParseError>) {
    let mut walker = SexprWalk::new(text);

    let root = ast.root_node();
    let mut iter = root.preorder();
    walker.walk(&mut iter);

    walker.parse.drain()
}

/// Given an ast with absolutely no ERROR or MISSING nodes, let's walk that tree
/// and collect our `trivia` and `events`.
struct SexprWalk<'src> {
    text: &'src str,
    parse: SexprParse,
    last_end: TextSize,
    between_two_tokens: bool,
}

struct SexprParse {
    events: Vec<Event<SexprSyntaxKind>>,
    trivia: Vec<Trivia>,
    errors: Vec<ParseError>,
}

impl<'src> SexprWalk<'src> {
    fn new(text: &'src str) -> Self {
        Self {
            text,
            parse: SexprParse::new(),
            last_end: TextSize::from(0),
            // Starts between the start of file and the first token
            between_two_tokens: false,
        }
    }

    fn walk(&mut self, iter: &mut Preorder) {
        while let Some(event) = iter.next() {
            match event {
                WalkEvent::Enter(node) => self.handle_enter(node, node.syntax_kind(), iter),
                WalkEvent::Leave(node) => self.handle_leave(node, node.syntax_kind()),
            }
        }
    }

    /// Walk only the upcoming node, including its subtree
    ///
    /// If the `next()` event is an `Enter`, `walk_next()` walks the
    /// `node` returned in the `Enter` event until hitting the `Leave` event
    /// of that `node`.
    fn walk_next(&mut self, iter: &mut Preorder) {
        let end = match iter.peek() {
            Some(end) => match end {
                WalkEvent::Enter(end) => *end,
                WalkEvent::Leave(_) => return,
            },
            None => return,
        };

        while let Some(event) = iter.next() {
            match event {
                WalkEvent::Enter(node) => self.handle_enter(node, node.syntax_kind(), iter),
                WalkEvent::Leave(node) => {
                    self.handle_leave(node, node.syntax_kind());
                    if node == end {
                        break;
                    }
                }
            }
        }
    }
    
    fn handle_enter(&mut self, node: tree_sitter::Node, kind: SexprSyntaxKind, iter: &mut Preorder) {
        println!("Enter Node: {:?}, {:?}", node, kind);
        match kind {
            SexprSyntaxKind::SEXPR_ROOT => self.handle_root_enter(),
            SexprSyntaxKind::SEXPR_SYMBOL_VALUE => self.handle_value_enter(kind),
            SexprSyntaxKind::SEXPR_LIST_VALUE => self.handle_list_enter(node, iter),

            // Tokens are no-ops on `Enter`, handled on `Leave`
            SexprSyntaxKind::L_PAREN | SexprSyntaxKind::R_PAREN => {},
            // SexprSyntaxKind::SEXPR_LIST => self
            _ => {
                panic!("Unhandled node kind: {:?}", kind);
            }
        }
    }

    fn handle_leave(&mut self, node: tree_sitter::Node, kind: SexprSyntaxKind) {
        println!("Leave Node: {:?}, {:?}", node, kind);
        match kind {
            SexprSyntaxKind::SEXPR_ROOT => self.handle_root_leave(node),
            SexprSyntaxKind::SEXPR_SYMBOL_VALUE => 
                self.handle_value_leave(node, kind, SexprSyntaxKind::SEXPR_SYMBOL_LITERAL),
            SexprSyntaxKind::SEXPR_LIST_VALUE => 
                self.handle_list_leave(),
                // SexprSyntaxKind::SEXPR_LIST
            SexprSyntaxKind::L_PAREN | SexprSyntaxKind::R_PAREN => {
                self.handle_token(node, kind);
            },
            _ => {
                panic!("Unhandled node kind: {:?}", kind);
            }
        }
    }

    fn handle_list_enter(&mut self, node: tree_sitter::Node, iter: &mut Preorder) {
        self.handle_node_enter(SexprSyntaxKind::SEXPR_LIST_VALUE);
        println!("Handling List Enter");

        while let Some(event) = iter.peek() {
            match event {
                WalkEvent::Enter(next) => match next.syntax_kind() {
                    SexprSyntaxKind::L_PAREN => {
                        self.walk_next(iter);
                        self.handle_node_enter(SexprSyntaxKind::SEXPR_LIST);
                    }
                    SexprSyntaxKind::R_PAREN => {
                        self.handle_node_leave(SexprSyntaxKind::SEXPR_LIST);
                        self.walk_next(iter);
                    }
                    SexprSyntaxKind::SEXPR_SYMBOL_VALUE => {
                        self.walk_next(iter);
                    }
                    // SexprSyntaxKind::R_PARAMETER | SexprSyntaxKind::COMMA | SexprSyntaxKind::COMMENT => {
                    //     self.walk_next(iter)
                    // }
                    kind => unreachable!("{kind:?}"),
                },
                WalkEvent::Leave(next) => {
                    if node != *next {
                        panic!("Expected next `Leave` event to be for `node`.");
                    }
                    break;
                }
            }
        }
    }

    fn handle_list_leave(&mut self) {
        self.handle_node_leave(SexprSyntaxKind::SEXPR_LIST);
    }

    fn handle_root_enter(&mut self) {
        // Start the overarching root
        self.handle_node_enter(SexprSyntaxKind::SEXPR_ROOT);

        // TODO: Handle optional BOM?
    }

    fn handle_root_leave(&mut self, node: tree_sitter::Node) {
        // Finish the overarching root

        // No longer between two tokens.
        // Now between last token and EOF.
        self.between_two_tokens = false;

        // TODO!: Don't unwrap()
        let this_end = TextSize::try_from(node.end_byte()).unwrap();
        let gap = &self.text[usize::from(self.last_end)..usize::from(this_end)];

        println!("this_end: {:?}", this_end);
        println!("gap: {:?}", gap);

        // Derive trivia between last token and end of document.
        // It is always leading trivia of the `EOF` token,
        // which `TreeSink` adds for us.
        self.parse
            .derive_trivia(gap, self.last_end, self.between_two_tokens);

        self.handle_node_leave(SexprSyntaxKind::SEXPR_ROOT);
    }

    fn handle_node_enter(&mut self, kind: SexprSyntaxKind) {
        self.parse.start(kind);
    }

    // `_kind` is nice to see at call sites as it makes the code more self-expanatory
    fn handle_node_leave(&mut self, _kind: SexprSyntaxKind) {
        self.parse.finish();
    }

    fn handle_value_enter(&mut self, kind: SexprSyntaxKind) {
        self.handle_node_enter(kind);
    }

    fn handle_value_leave(
        &mut self,
        node: tree_sitter::Node,
        kind: SexprSyntaxKind,
        literal_kind: SexprSyntaxKind,
    ) {
        // Push the token for the literal
        self.handle_token(node, literal_kind);

        // Then close the node
        self.handle_node_leave(kind);
    }

    fn handle_token(&mut self, node: tree_sitter::Node, kind: SexprSyntaxKind) {
        // TODO!: Don't unwrap()
        let this_start = TextSize::try_from(node.start_byte()).unwrap();
        let this_end = TextSize::try_from(node.end_byte()).unwrap();
        let gap = &self.text[usize::from(self.last_end)..usize::from(this_start)];

        self.parse
            .derive_trivia(gap, self.last_end, self.between_two_tokens);

        self.parse.token(kind, this_end);

        self.last_end = this_end;
        self.between_two_tokens = true;
    }
}

impl SexprParse {
    fn new() -> Self {
        Self {
            events: Vec::new(),
            trivia: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn start(&mut self, kind: SexprSyntaxKind) {
        println!("pushing Start event {:?}", kind);
        self.push_event(Event::Start {
            kind,
            forward_parent: None,
        });
    }

    fn push_event(&mut self, event: Event<SexprSyntaxKind>) {
        self.events.push(event);
    }

    fn push_trivia(&mut self, trivia: Trivia) {
        self.trivia.push(trivia);
    }

    fn finish(&mut self) {
        println!("pushing Finish event");
        self.push_event(Event::Finish);
    }

    fn drain(self) -> (Vec<Event<SexprSyntaxKind>>, Vec<Trivia>, Vec<ParseError>) {
        (self.events, self.trivia, self.errors)
    }

    fn token(&mut self, kind: SexprSyntaxKind, end: TextSize) {
        self.push_event(Event::Token { kind, end });
    }

    fn derive_trivia(&mut self, text: &str, mut start: TextSize, between_two_tokens: bool) {
        let mut iter = text.as_bytes().iter().peekable();
        let mut end = start;

        // - Between the start of file and the first token, all trivia is leading
        //   (it leads the first token), so we skip this.
        // - Between the last token and the end of file, all trivia is leading
        //   (it leads the EOF token that `TreeSink` adds), so we skip this.
        // - Between two tokens, all trivia is leading unless we see a newline,
        //   which this branch handles specially.
        if between_two_tokens {
            let mut trailing = false;

            // All whitespace between two tokens is leading until we hit the
            // first `\r`, `\r\n`, or `\n`, at which point the whitespace is
            // considered trailing of the last token, and the newline and
            // everything after it is considered leading of the next token.
            // A lone `\r` not attached to an `\n` should not happen in a
            // well-formed file (unless inside a string token), so we just
            // treat it as a `\r\n` line ending.
            while let Some(byte) = iter.peek() {
                if let b'\r' | b'\n' = byte {
                    // We found a newline, so all trivia up to this point is
                    // trailing to the last token. Don't advance the iterator so
                    // that this newline may be processed as leading trivia.
                    trailing = true;

                    // Break and fallthrough
                    break;
                }
                end += TextSize::from(1);
                let _ = iter.next();
            }

            // Push the range of whitespace
            // TODO(semicolon): This currently includes semicolons as whitespace
            if start != end {
                let range = TextRange::new(start, end);
                self.push_trivia(Trivia::new(TriviaPieceKind::Whitespace, range, trailing));
                start = end;
            }

            // Fallthrough so that our current byte can be processed as leading
            // trivia
        }

        // Now push all leading trivia
        let trailing = false;

        while let Some(byte) = iter.next() {
            end += TextSize::from(1);

            if Self::is_whitespace(*byte) {
                // Finish out stream of whitespace
                while iter.next_if(|byte| Self::is_whitespace(**byte)).is_some() {
                    end += TextSize::from(1);
                }
                let range = TextRange::new(start, end);
                self.push_trivia(Trivia::new(TriviaPieceKind::Whitespace, range, trailing));
                start = end;
                continue;
            }

            if let b'\r' = byte {
                match iter.next_if(|byte| **byte == b'\n') {
                    Some(_) => {
                        // Finish out `\r\n`
                        end += TextSize::from(1);
                        let range = TextRange::new(start, end);
                        self.push_trivia(Trivia::new(TriviaPieceKind::Newline, range, trailing));
                        start = end;
                    }
                    None => {
                        // Finish out `\r`
                        let range = TextRange::new(start, end);
                        self.push_trivia(Trivia::new(TriviaPieceKind::Newline, range, trailing));
                        start = end;
                    }
                }
                continue;
            }

            if let b'\n' = byte {
                // Finish out `\n`
                let range = TextRange::new(start, end);
                self.push_trivia(Trivia::new(TriviaPieceKind::Newline, range, trailing));
                start = end;
                continue;
            }

            unreachable!("Detected non trivia character!");
        }
    }

    fn is_whitespace(byte: u8) -> bool {
        // `WHS` maps newlines as "whitespace" but we handle that specially
        match biome_unicode_table::lookup_byte(byte) {
            Dispatch::WHS => byte != b'\r' && byte != b'\n',
            // TODO(semicolon): Mapping semicolons as whitespace until we handle them officially
            Dispatch::SEM => true,
            _ => false,
        }
    }
}