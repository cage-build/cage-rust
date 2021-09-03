mod parser;
use super::lexer::Lexer;
use super::{ConfigurationError, Position};
pub use parser::Parser;

/// One statement from the file
#[derive(Debug, PartialEq)]
pub enum Statement {
    /// A statement tag
    Tag(Position, String),
    /// An empty line
    EmptyLine,
    /// A comment
    Comment(String),
}

/// Parse the configuration file. After fail always return `None`.
pub fn parse(
    config_content: &str,
) -> impl Iterator<Item = Result<Statement, ConfigurationError>> + '_ {
    let l = Lexer::new(config_content).map(|i| Ok(i));
    Parser::new(l)
}
