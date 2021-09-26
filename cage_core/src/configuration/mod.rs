#[allow(dead_code)]
mod error;
#[allow(dead_code)]
mod lexer;
#[allow(dead_code)]
mod statement;
#[allow(dead_code)]
mod version;

#[allow(unused_imports)]
use version::Version;

use std::fmt;

pub use error::ConfigurationError;

/// The result of the lexer.
type TokenResult = Result<(Position, lexer::Word), ConfigurationError>;

/// The position of one object in the configuration file.
#[derive(Debug, Copy, Clone, std::cmp::PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}
impl Position {
    #[cfg(test)]
    const ZERO: Position = Position { line: 0, column: 0 };
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line:{}, column:{}", self.line, self.column)
    }
}
