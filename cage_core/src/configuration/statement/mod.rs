mod blob;
mod generator;
mod parser;
use super::lexer::{escape, Lexer, Word};
use super::{ConfigurationError, Position, TokenResult};
use std::iter::Peekable;

/// One statement from the file
#[derive(Debug, PartialEq)]
pub enum Statement {
    /// A directory variable declaration
    Directory(Position, Identifier, Blob),
    /// A file variable declaration
    File(Position, Identifier, Blob),
    /// A statement tag
    Tag(Position, String),
    /// A generator declaration.
    Generator(Position, String, Vec<Generator>),
}

#[derive(Debug, PartialEq)]
pub enum Identifier {
    /// Simple variable name.
    Variable(String),
    /// The system variable for package, `$pkg`.
    SystemPackage,
    /// The system variable for executable binary, `$run`.
    SystemRun,
    /// The system variable for executable test, `$test`.
    SystemTest,
}

#[derive(Debug, PartialEq)]
pub struct Blob {
    position: Position,
    value: BlobValue,
    pipes: Vec<Generator>,
}

#[derive(Debug, PartialEq)]
pub enum BlobValue {
    Name(Name),
    Literal(String),
    Concatenation(Vec<Blob>),
    Composition(Vec<(Position, String, Blob)>),
    Parenthesis(Box<Blob>),
}

#[derive(Debug, PartialEq)]
pub struct Generator {
    /// The position of the value of the generator.
    position: Position,
    /// The input is a directory or a file.
    input_is_dir: bool,
    /// The name given
    name: Option<String>,
    /// The value of the generator.
    generator: BlobValue,
    /// Arguments for the generator.
    args: Vec<(Position, String)>,
}

#[derive(Debug, PartialEq)]
pub enum Name {
    /// The name of a generator varibale.
    Variable(String),
    /// A file or a repository from the source filesystem.
    Source(String),
    /// An external generator URL.
    Url(String),
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
    let l = Lexer::new(config_content).map(|r| match r {
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

/// Create a [`ConfigurationError::UnexpectedToken`] into a Result::Err.
fn unexpected_token<T>(p: Position, w: Word, op: &'static str) -> Result<T, ConfigurationError> {
    Err(ConfigurationError::UnexpectedToken(
        p,
        format!("{:?}", w),
        op,
    ))
}

#[test]
fn test_parser() {
    fn pos(line: usize) -> Position {
        Position { line, column: 1 }
    }

    let mut parser = test_value(vec![
        Word::KeywordDir,
        Word::SystemPackage,
        Word::DirectoryComposeOpen,
        // first element
        Word::QuotedString("style.css".to_string()),
        Word::Colon,
        Word::QuotedString("styling/".to_string()),
        Word::PipeDirectory,
        Word::DollardString("https://styling.io/v1/gen.wasm".to_string()),
        Word::Comma,
        // second element: directory
        Word::QuotedString("img/".to_string()),
        Word::Colon,
        Word::QuotedString("img-src/".to_string()),
        Word::PipeDirectory,
        Word::ParenthesisOpen,
        Word::QuotedString("build/resize.c".to_string()),
        Word::PipeFile,
        Word::DollardString("https://c.land/2.wasm".to_string()),
        Word::ParenthesisClose,
        Word::PipeDirectory,
        Word::DollardString("https://image.io/v1/grey.wasm".to_string()),
        Word::Comma,
        Word::DirectoryComposeClose,
        // filal pipe
        Word::PipeDirectory,
        Word::SimpleString("minifier".to_string()),
        Word::QuotedString("arg1".to_string()),
        Word::QuotedString("arg2".to_string()),
    ]);

    let expected = Statement::Directory(
        pos(0),
        Identifier::SystemPackage,
        Blob {
            position: pos(2),
            value: BlobValue::Composition(vec![
                (
                    pos(3),
                    "style.css".to_string(),
                    Blob {
                        position: pos(5),
                        value: BlobValue::Name(Name::Source("styling/".to_string())),
                        pipes: vec![Generator {
                            position: pos(6),
                            input_is_dir: true,
                            name: None,
                            generator: BlobValue::Name(Name::Url(
                                "https://styling.io/v1/gen.wasm".to_string(),
                            )),
                            args: Vec::new(),
                        }],
                    },
                ),
                (
                    pos(9),
                    "img/".to_string(),
                    Blob {
                        position: pos(11),
                        value: BlobValue::Name(Name::Source("img-src/".to_string())),
                        pipes: vec![
                            Generator {
                                position: pos(12),
                                input_is_dir: true,
                                name: None,
                                generator: BlobValue::Parenthesis(Box::new(Blob {
                                    position: pos(13),
                                    value: BlobValue::Name(Name::Source(
                                        "build/resize.c".to_string(),
                                    )),
                                    pipes: vec![Generator {
                                        position: pos(15),
                                        input_is_dir: false,
                                        name: None,
                                        generator: BlobValue::Name(Name::Url(
                                            "https://c.land/2.wasm".to_string(),
                                        )),
                                        args: Vec::new(),
                                    }],
                                })),
                                args: Vec::new(),
                            },
                            Generator {
                                position: pos(18),
                                input_is_dir: true,
                                name: None,
                                generator: BlobValue::Name(Name::Url(
                                    "https://image.io/v1/grey.wasm".to_string(),
                                )),
                                args: Vec::new(),
                            },
                        ],
                    },
                ),
            ]),
            pipes: vec![Generator {
                position: pos(22),
                input_is_dir: true,
                name: None,
                generator: BlobValue::Name(Name::Variable("minifier".to_string())),
                args: vec![(pos(24), "arg1".to_string()), (pos(25), "arg2".to_string())],
            }],
        },
    );
    let received = parser.next().unwrap().unwrap();

    assert_print(&expected, &received);
}

/// If left != right, print into the stderr the differrence in red color.
#[cfg(test)]
fn assert_print<T: PartialEq + std::fmt::Debug>(left: &T, right: &T) {
    if left != right {
        let left_string = format!("{:#?}", left);
        let right_string = format!("{:#?}", right);
        let width = left_string.lines().map(|s| s.len()).max().unwrap_or(1);
        let mut left_lines = left_string.lines();
        let mut right_lines = right_string.lines();

        let mut next = || -> Option<(&str, &str)> {
            match (left_lines.next(), right_lines.next()) {
                (Some(l), Some(r)) => Some((l, r)),
                (None, Some(r)) => Some(("", r)),
                (Some(l), None) => Some((l, "")),
                (None, None) => None,
            }
        };

        while let Some((l, r)) = next() {
            match l == r {
                true => eprintln!("{:2$} ¦ {:2$}", l, r, width),
                false => eprintln!("\x1b[1;31m{:2$} | {:2$}\x1b[0m", l, r, width),
            }
        }

        panic!("not equal");
    }
}
