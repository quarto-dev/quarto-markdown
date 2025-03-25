mod error;
mod options;
mod parse;

#[allow(unused)]
mod treesitter;

use quarto_markdown_factory::MarkdownSyntaxFactory;
pub use error::ParseError;
pub use options::MarkdownParserOptions;
pub use parse::parse;
pub use parse::parse_markdown_with_cache;
pub use parse::Parse;

use quarto_markdown_syntax::MarkdownLanguage;
use biome_parser::tree_sink::LosslessTreeSink;

pub(crate) type MarkdownLosslessTreeSink<'source> = LosslessTreeSink<'source, SexprLanguage, SexprSyntaxFactory>;
