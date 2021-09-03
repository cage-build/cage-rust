mod parser;
use super::Position;
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
