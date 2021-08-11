use std::{error, fmt};

/// Low level error from configuration parsing.
#[derive(Debug)]
pub enum Error {
    /// The version was not found from configuration file.
    VersionNotFound,
    /// The version found from configuration file is unknown.
    VersionUnknown(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VersionNotFound => f.write_str("Version not found"),
            Self::VersionUnknown(v) => write!(f, "The version {:?} is unknown", v),
        }
    }
}

impl error::Error for Error {}
