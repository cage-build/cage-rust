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

    /// Parse `{ string_quoted ":" blob "," } [ string_quoted ":" blob ] "}"`
    fn parse_composition(&mut self) -> Result<BlobValue, ConfigurationError> {
        if self.peek() == Some(&Word::DirectoryComposeClose) {
            self.next_expected()?;
            return Ok(BlobValue::Composition(Vec::new()));
        }

        let mut values = Vec::new();
        loop {
            let (position, name) = match self.next_expected()? {
                (p, Word::QuotedString(n)) => (p, n),
                (p, w) => unexpected_token(p, w, "composition.name")?,
            };

            match self.next_expected()? {
                (_, Word::Colon) => {}
                (p, w) => unexpected_token(p, w, "composition.collon")?,
            };

            let blob = self.parse_blob()?;
            values.push((position, name, blob));

            let (p, next) = self.next_expected()?;
            match next {
                Word::DirectoryComposeClose => break,
                Word::Comma if self.peek() == Some(&Word::DirectoryComposeClose) => break,
                Word::Comma => {}
                w => unexpected_token(p, w, "composition.end_item")?,
            }
        }
        Ok(BlobValue::Composition(values))
    }

    /// Parse `{ blob "," } [ blob ] "]"`
    fn parse_concatenation(&mut self) -> Result<BlobValue, ConfigurationError> {
        if self.peek() == Some(&Word::DirectoryConcatClose) {
            self.next_expected()?;
            return Ok(BlobValue::Concatenation(Vec::new()));
        }

        let mut values = Vec::new();
        loop {
            values.push(self.parse_blob()?);

            let (p, w) = self.next_expected()?;
            match w {
                Word::DirectoryConcatClose => break,
                Word::Comma if self.peek() == Some(&Word::DirectoryConcatClose) => {
                    self.next_expected()?;
                    break;
                }
                Word::Comma => {}
                _ => unexpected_token(p, w, "concatenation.next_item")?,
            };
        }
        Ok(BlobValue::Concatenation(values))
    }
}

#[test]
fn parser_blob_name() {
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
#[test]
fn parse_composition() {
    use super::super::Position;
    let mut parser = super::test_value(vec![
        Word::DirectoryComposeClose,
        Word::QuotedString("key".to_string()),
        Word::Colon,
        Word::SimpleString("var".to_string()),
        Word::Comma,
        Word::DirectoryComposeClose,
    ]);

    assert_eq!(
        BlobValue::Composition(vec![]),
        parser.parse_composition().unwrap()
    );
    assert_eq!(
        BlobValue::Composition(vec![(
            Position { line: 1, column: 1 },
            "key".to_string(),
            Blob {
                position: Position { line: 3, column: 1 },
                value: BlobValue::Name(Name::Variable("var".to_string()))
            }
        )]),
        parser.parse_composition().unwrap()
    );
}
#[test]
fn parser_concatenation() {
    use super::super::Position;
    let mut parser = super::test_value(vec![
        Word::DirectoryConcatClose,
        Word::SimpleString("var".to_string()),
        Word::Comma,
        Word::DirectoryConcatClose,
        Word::SimpleString("var1".to_string()),
        Word::Comma,
        Word::SimpleString("var2".to_string()),
        Word::DirectoryConcatClose,
    ]);

    assert_eq!(
        BlobValue::Concatenation(vec![]),
        parser.parse_concatenation().unwrap()
    );
    assert_eq!(
        BlobValue::Concatenation(vec![Blob {
            position: Position { line: 1, column: 1 },
            value: BlobValue::Name(Name::Variable("var".to_string())),
        },]),
        parser.parse_concatenation().unwrap()
    );
    assert_eq!(
        BlobValue::Concatenation(vec![
            Blob {
                position: Position { line: 4, column: 1 },
                value: BlobValue::Name(Name::Variable("var1".to_string())),
            },
            Blob {
                position: Position { line: 6, column: 1 },
                value: BlobValue::Name(Name::Variable("var2".to_string())),
            },
        ]),
        parser.parse_concatenation().unwrap()
    );
}
