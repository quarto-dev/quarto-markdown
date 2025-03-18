mod error;
mod options;
mod parse;

#[allow(unused)]
mod treesitter;

use quarto_sexpr_factory::SexprSyntaxFactory;
pub use error::ParseError;
pub use options::SexprParserOptions;
pub use parse::parse;
pub use parse::parse_sexpr_with_cache;
pub use parse::Parse;

use quarto_sexpr_syntax::SexprLanguage;
use biome_parser::tree_sink::LosslessTreeSink;

pub(crate) type SexprLosslessTreeSink<'source> = LosslessTreeSink<'source, SexprLanguage, SexprSyntaxFactory>;
