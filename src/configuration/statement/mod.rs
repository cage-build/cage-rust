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
    args: Vec<String>,
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
    let l = Lexer::new(config_content).map(|i| Ok(i));
    Parser::new(l)
}
