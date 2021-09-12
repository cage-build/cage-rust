mod char_iter;
mod error;
mod escape;
mod iterator;

use super::Position;
use char_iter::CharItem;
pub use error::LexerError;
pub use escape::escape;
use iterator::State;

/// The Lexer, split the input into [`Word`]. It's an iterator.
pub struct Lexer<'a> {
    chars: CharItem<'a>,
    state: State,
    /// The content of the current comment, varibale name, ...
    buff: String,
    /// For founded element, send at the comming call of `next` method.
    comming: Option<Word>,
    /// The founed error.
    error: Option<LexerError>,
}

/// One lexer token. Created with [`Lexer.next()`].
#[derive(Debug, PartialEq)]
pub enum Word {
    /// "tag" keyword
    KeywordTag,
    /// "dir" keyword
    KeywordFile,
    /// "file" keyword
    KeywordDir,

    /// The system variable for package, `$pkg`.
    SystemPackage,
    /// The system variable for executable binary, `$run`.
    SystemRun,
    /// The system variable for executable test, `$test`.
    SystemTest,

    /// A simple unqoted string, used for variable and generator args.
    SimpleString(String),
    /// File path. It does not contain the limit quotes but it is not unescaped.
    QuotedString(String),
    /// A literal string, can be an url or a value used as a file content.
    /// It does not contain the limit quotes but it is not unescaped.
    DollardString(String),

    /// Colon, to separate the key and the value, in directory.
    Colon,
    /// The coma, to separate several elements in a directory composition or aggregation.
    Comma,
    /// The default generator operator, `??`
    DefaultGenerator,
    /// The pipe to a file.
    PipeFile,
    /// The pipe to a directory
    PipeDirectory,
    /// Opening Symbol for composie a directory. `{`
    DirectoryComposeOpen,
    /// Closing Symbol for composie a directory. `}`
    DirectoryComposeClose,
    /// Closing Symbol for concatenation a directory. `]`
    DirectoryConcatOpen,
    /// Closing Symbol for concatenation a directory. `]`
    DirectoryConcatClose,

    /// A comment, used to format the build config file.
    /// Do not contain the hash `#` and the line return.
    Comment(String),
    /// An new line, used to format.
    NewLine,
}

#[test]
fn test_lexer() {
    let mut l = Lexer::new(
        r##"
# A comment
tag superTag

file front "front/" > "min" ?? $"https://exemple.com/minifier"

dir$pkg[
	{
		"file.txt": $"A great literal string.
Enclose by double quote \"\".",
	},
	variable,
]
"##,
    );
    let mut next = || l.next().unwrap().1;

    assert_eq!(Word::NewLine, next());
    assert_eq!(Word::Comment(" A comment".to_string()), next());

    assert_eq!(Word::KeywordTag, next());
    assert_eq!(Word::SimpleString("superTag".to_string()), next());
    assert_eq!(Word::NewLine, next());
    assert_eq!(Word::NewLine, next());

    assert_eq!(Word::KeywordFile, next());
    assert_eq!(Word::SimpleString("front".to_string()), next());
    assert_eq!(Word::QuotedString("front/".to_string()), next());
    assert_eq!(Word::PipeDirectory, next());
    assert_eq!(Word::QuotedString("min".to_string()), next());
    assert_eq!(Word::DefaultGenerator, next());
    assert_eq!(
        Word::DollardString("https://exemple.com/minifier".to_string()),
        next()
    );
    assert_eq!(Word::NewLine, next());
    assert_eq!(Word::NewLine, next());

    assert_eq!(Word::KeywordDir, next());
    assert_eq!(Word::SystemPackage, next());
    assert_eq!(Word::DirectoryConcatOpen, next());
    assert_eq!(Word::NewLine, next());

    assert_eq!(Word::DirectoryComposeOpen, next());
    assert_eq!(Word::NewLine, next());

    assert_eq!(Word::QuotedString("file.txt".to_string()), next());
    assert_eq!(Word::Colon, next());
    assert_eq!(
        Word::DollardString(
            "A great literal string.\nEnclose by double quote \\\"\\\".".to_string()
        ),
        next()
    );
    assert_eq!(Word::Comma, next());
    assert_eq!(Word::NewLine, next());

    assert_eq!(Word::DirectoryComposeClose, next());
    assert_eq!(Word::Comma, next());
    assert_eq!(Word::NewLine, next());

    assert_eq!(Word::SimpleString("variable".to_string()), next());
    assert_eq!(Word::Comma, next());
    assert_eq!(Word::NewLine, next());

    assert_eq!(Word::DirectoryConcatClose, next());
    assert_eq!(Word::NewLine, next());

    assert_eq!(None, l.next());
    assert_eq!(Ok(()), l.err());
}
