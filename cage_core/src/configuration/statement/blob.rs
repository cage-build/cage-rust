use super::super::lexer::Word;
use super::super::ConfigurationError;
use super::{unexpected_token, Blob, BlobValue, Name, Parser, TokenResult};

impl<I> Parser<I>
where
    I: Iterator<Item = TokenResult>,
{
    fn parse_blob(&mut self) -> Result<Blob, ConfigurationError> {
        let (position, word) = self.next_expected()?;

        let value = match word {
            Word::SimpleString(v) => BlobValue::Name(Name::Variable(v)),
            Word::QuotedString(f) => BlobValue::Name(Name::Source(f)),
            Word::DollardString(s) => BlobValue::Literal(s),
            Word::SystemRun => BlobValue::Name(Name::SystemRun),
            Word::SystemTest => BlobValue::Name(Name::SystemTest),
            Word::SystemPackage => BlobValue::Name(Name::SystemPackage),
            Word::DirectoryConcatOpen => unimplemented!(),
            Word::DirectoryComposeOpen => unimplemented!(),
            w => unexpected_token(position, w, "blob")?,
        };

        Ok(Blob { value, position })
    }
}

#[test]
fn parser_blob() {
    use super::super::Position;
    let mut parser = super::test_value(vec![
        Word::SimpleString("var".to_string()),
        Word::QuotedString(".".to_string()),
        Word::DollardString("https://exemple.com/foo.zip".to_string()),
        Word::SystemRun,
        Word::SystemTest,
        Word::SystemPackage,
    ]);

    assert_eq!(
        Blob {
            position: Position { line: 0, column: 1 },
            value: BlobValue::Name(Name::Variable("var".to_string())),
        },
        parser.parse_blob().unwrap()
    );
    assert_eq!(
        Blob {
            position: Position { line: 1, column: 1 },
            value: BlobValue::Name(Name::Source(".".to_string())),
        },
        parser.parse_blob().unwrap()
    );
    assert_eq!(
        Blob {
            position: Position { line: 2, column: 1 },
            value: BlobValue::Literal("https://exemple.com/foo.zip".to_string()),
        },
        parser.parse_blob().unwrap()
    );
    assert_eq!(
        Blob {
            position: Position { line: 3, column: 1 },
            value: BlobValue::Name(Name::SystemRun),
        },
        parser.parse_blob().unwrap()
    );
    assert_eq!(
        Blob {
            position: Position { line: 4, column: 1 },
            value: BlobValue::Name(Name::SystemTest),
        },
        parser.parse_blob().unwrap()
    );
    assert_eq!(
        Blob {
            position: Position { line: 5, column: 1 },
            value: BlobValue::Name(Name::SystemPackage),
        },
        parser.parse_blob().unwrap()
    );
}
