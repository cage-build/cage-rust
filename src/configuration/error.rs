use super::{lexer::LexerError, Position};
use std::{error::Error, fmt};

/// Low level error from configuration parsing.
#[derive(Debug)]
pub enum ConfigurationError {
    /// The version was not found from configuration file.
    VersionNotFound,
    /// The version found from configuration file is unknown.
    VersionUnknown(String),
    /// An error ocure when tokenize the configuration file.
    Lexer((Position, LexerError)),
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VersionNotFound => f.write_str("Version not found"),
            Self::VersionUnknown(v) => write!(f, "The version {:?} is unknown", v),
            Self::Lexer((Position { line, column }, _)) => {
                write!(f, "Lexer error at line {} column {}", line, column)
            }
        }
    }
}

impl Error for ConfigurationError {}
