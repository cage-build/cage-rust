#[allow(dead_code)]
mod error;
#[allow(dead_code)]
mod lexer;
#[allow(dead_code)]
mod tree;
#[allow(dead_code)]
mod version;

#[allow(unused_imports)]
use lexer::lexer;
#[allow(unused_imports)]
use version::Version;

pub use error::Error;

/// The position of one object in the configuration file.
#[derive(Debug, Copy, Clone, std::cmp::PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}
