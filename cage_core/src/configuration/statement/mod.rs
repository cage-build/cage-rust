mod parser;
use super::lexer::{escape, Lexer, Word};
use super::{ConfigurationError, Position};
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
}

#[derive(Debug, PartialEq)]
pub enum FileValue {
    Literal(String),
    Name(String),
    Variable(Variable),
}

#[derive(Debug, PartialEq)]
pub struct Generator {
    position: Position,
    name: Option<String>,
    generator: GeneratorValue,
    args: Vec<(Position, String)>,
}

#[derive(Debug, PartialEq)]
pub enum GeneratorValue {
    /// A file of generator variable.
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

type TokenResult = Result<(Position, Word), ConfigurationError>;

#[derive(Debug, Copy, Clone)]
enum State {
    /// Initial state
    Initial,
    /// Like Initial but if [`Word::NewLine`] is the next token else as Initial.
    WaitNewLine,
    /// Wait the tag name
    WaitTag,
}

// An iterator of [`Statement`] from an iterator of [`Word`].
struct Parser<I: Iterator<Item = TokenResult>> {
    source: Peekable<I>,
    state: State,
}
