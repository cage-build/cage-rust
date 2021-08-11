use std::fmt;

#[derive(Debug)]
pub enum Error {
    VersionNotFound,
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

impl std::error::Error for Error {}
