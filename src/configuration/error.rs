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
        }
    }
}

impl Error for ConfigurationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::VersionNotFound | Self::VersionUnknown(_) | Self::ParserExpectedTagName(_) => {
                None
            }
            Self::Lexer(_, err) => Some(err),
        }
    }
}
