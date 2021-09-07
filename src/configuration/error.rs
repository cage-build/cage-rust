use super::{lexer::LexerError, Position};
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// Low level error from configuration parsing.
#[derive(Debug, PartialEq)]
pub enum ConfigurationError {
    /// The version was not found from configuration file.
    VersionNotFound,
    /// The version found from configuration file is unknown.
    VersionUnknown(String),
    /// An error ocure when tokenize the configuration file.
    Lexer(Position, LexerError),
    /// We expect a tag name after.
    ParserExpectedTagName(Position),
    /// The parser can not use this generator kind.
    ParserWrongGeneratorToken(Position, String),

    ParserGeneratorNameToken(Position, String),
    /// We do not end the parsing of this statement.
    UnexpectedStatementEnd(Position),

    UnexpectedEnd,
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Self::VersionNotFound => f.write_str("Version not found"),
            Self::VersionUnknown(v) => write!(f, "The version {:?} is unknown", v),
            Self::Lexer(p, _) => write!(f, "{}, Lexer fail", p),
            Self::ParserExpectedTagName(p) => {
                write!(f, "{}, Expected a tag name after `tag` keyword.", p)
            }
            Self::ParserWrongGeneratorToken(p, s) => write!(
                f,
                "{}, the parser can not create a generator from this token {}",
                p, s
            ),
            Self::ParserGeneratorNameToken(p, s) => {
                write!(f, "{}, can not get generator name from token {}", p, s)
            }
            Self::UnexpectedStatementEnd(p) => {
                write!(f, "{}, unexpected end of this statement.", p)
            }
            Self::UnexpectedEnd => f.write_str("Unexpected end when parse a statement"),
        }
    }
}

impl Error for ConfigurationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::VersionNotFound
            | Self::VersionUnknown(_)
            | Self::ParserExpectedTagName(_)
            | Self::ParserWrongGeneratorToken(_, _)
            | Self::ParserGeneratorNameToken(_, _)
            | Self::UnexpectedStatementEnd(_)
            | Self::UnexpectedEnd => None,
            Self::Lexer(_, err) => Some(err),
        }
    }
}
