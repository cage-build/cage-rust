use std::{error::Error, fmt};

/// A lexer error.
#[derive(Debug, PartialEq, Clone)]
pub enum LexerError {
    /// Neighter a symbol char nor a aplhanumeric characters.
    UnknowChar(char),
    /// A not closed file path or literal string.
    StringWithoutEnd,
    /// A half or default generator symbol (just one `?`).
    HalfDefaultGenerator,
    /// A unknown system variable.
    UnknowSystem(String),
    /// Double dollard, unknow token.
    DoubleDollard,
    /// A dollard at end of the configuration file, expected a literal string or a system variable.
    DollardAtEOF,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnknowChar(c) => write!(
                f,
                "Unkown this char: {:?} (Neighter a symbol char nor a aplhanumeric characters)",
                c
            ),
            LexerError::StringWithoutEnd => f.write_str("A not closed file path or literal string"),
            LexerError::HalfDefaultGenerator => f.write_str("A single '?', unknown this symbol (maybe '??')."),
			LexerError::UnknowSystem(v) => write!(f, "Unknown the {:?} system variable", v),
			LexerError::DoubleDollard => f.write_str("Double dollard, unknown this token"),
			LexerError::DollardAtEOF => f.write_str("A dollard at end of the configuration file, expected a literal string or a system variable."),
        }
    }
}
impl Error for LexerError {}
