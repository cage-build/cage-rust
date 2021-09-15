mod generator;
mod parser;
use super::lexer::{escape, Lexer, Word};
use super::{ConfigurationError, Position, TokenResult};
use std::iter::Peekable;

/// One statement from the file
#[derive(Debug, PartialEq)]
pub enum Statement {
    /// A statement tag
    Tag(Position, String),
    /// An empty line
    EmptyLine,
    /// A comment
    Comment(String),
    /// A file variable
    File(Position, String, FileValue),
    /// A generator declaration.
    Generator(Position, String, Generator),
}

#[derive(Debug, PartialEq)]
pub enum FileValue {
    Literal(String),
    Name(String),
    Variable(Variable),
}

#[derive(Debug, PartialEq)]
pub struct Generator {
    /// The position of the value of the generator.
    position: Position,
    /// The name given
    name: Option<String>,
    /// The value of the generator.
    generator: GeneratorValue,
    /// Arguments for the generator.
    args: Vec<(Position, String)>,
}

#[derive(Debug, PartialEq)]
pub enum GeneratorValue {
    /// The name of a generator varibale.
    Variable(String),
    /// A file in the source filesystem.
    File(String),
    /// An external generator URL.
    Url(String),
}

#[derive(Debug, PartialEq)]
pub enum Variable {
    /// A standard variable.
    Variable(String),
    /// The system variable for package, `$pkg`.
    SystemPackage,
    /// The system variable for executable binary, `$run`.
    SystemRun,
    /// The system variable for executable test, `$test`.
    SystemTest,
}

/// Parse the configuration file. After fail always return `None`.
pub fn parse(
    config_content: &str,
) -> impl Iterator<Item = Result<Statement, ConfigurationError>> + '_ {
    let l = Lexer::new(config_content).map(|i| Ok(i)).map(|r| match r {
        Ok((p, Word::QuotedString(s))) => match escape(p, s) {
            Ok(s) => Ok((p, Word::QuotedString(s))),
            Err(e) => Err(e),
        },
        Ok((p, Word::DollardString(s))) => match escape(p, s) {
            Ok(s) => Ok((p, Word::DollardString(s))),
            Err(e) => Err(e),
        },
        _ => r,
    });
    Parser::new(l)
}

// An iterator of [`Statement`] from an iterator of [`Word`].
struct Parser<I: Iterator<Item = TokenResult>> {
    source: Peekable<I>,
}

/// Create an parser from a vec of word.
/// The position column is always 1, and the line is the index.
#[cfg(test)]
fn test_value(src: Vec<Word>) -> Parser<impl Iterator<Item = TokenResult>> {
    Parser::new(
        src.into_iter()
            .enumerate()
            .map(|(line, w)| Ok((Position { line, column: 1 }, w))),
    )
}
