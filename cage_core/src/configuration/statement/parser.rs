use super::super::lexer::Word;
use super::super::{ConfigurationError, Position};
use super::{unexpected_token, Blob, Identifier, Parser, Statement, TokenResult};

impl<I> Parser<I>
where
    I: Iterator<Item = TokenResult>,
{
    /// Take an iterator, the [`Word::DollardString`] and [`Word::QuotedString`] must be escaped.
    pub fn new(source: I) -> Self {
        Self {
            source: source.peekable(),
        }
    }

    /// Get Statement::Tag, the "tag" keyword is already consumed.
    pub fn parse_tag_statement(
        &mut self,
        position: Position,
    ) -> Result<Statement, ConfigurationError> {
        let name = self.get_statment_name()?;
        Ok(Statement::Tag(position, name))
    }

    /// Get the identifier for file and directory statement.
    fn get_statement_identifier(&mut self) -> Result<Identifier, ConfigurationError> {
        let (position, word) = self.next_expected()?;
        Ok(match word {
            Word::SimpleString(s) => Identifier::Variable(s),
            Word::SystemRun => Identifier::SystemRun,
            Word::SystemPackage => Identifier::SystemPackage,
            Word::SystemTest => Identifier::SystemTest,
            _ => unexpected_token(position, word, "statement identivfier")?,
        })
    }

    /// Get the content of a file of directory sstatement. `id blob`
    fn id_and_content(&mut self) -> Result<(Identifier, Blob), ConfigurationError> {
        Ok((self.get_statement_identifier()?, self.parse_blob()?))
    }

    /// Get the name after the tag of generator statment keyword.
    pub fn get_statment_name(&mut self) -> Result<String, ConfigurationError> {
        match self.next_expected()? {
            (_, Word::SimpleString(s)) => Ok(s),
            (p, w) => Err(ConfigurationError::ParserExpectedStatementName(
                p,
                format!("{:?}", w),
            )),
        }
    }

    /// Return the next word other. Skip [`Word::NewLine`] or [`Word::Comment`].
    /// If the next token is a keyword, returns [`Err(ConfigurationError::UnexpectedEnd)`] because
    /// this keyword begin a other statement, and this method must be used only by inside statement
    /// parsing methods.
    pub fn next_expected(&mut self) -> Result<(Position, Word), ConfigurationError> {
        match self.peek() {
            Some(
                &Word::KeywordDir
                | &Word::KeywordFile
                | &Word::KeywordGenerator
                | &Word::KeywordTag,
            ) => {
                return Err(ConfigurationError::UnexpectedEnd);
            }
            _ => {}
        };

        match self.source.next() {
            Some(Ok((_, Word::NewLine | Word::Comment(_)))) => self.next_expected(),
            Some(Ok((p, w))) => Ok((p, w)),
            Some(Err(e)) => Err(e),
            None => Err(ConfigurationError::UnexpectedEnd),
        }
    }

    /// Peek the next token other than a [`Word::NewLine`] of [`Word::Comment`]. Return None if error.
    pub fn peek(&mut self) -> Option<&Word> {
        while match self.source.peek() {
            Some(Ok((_, Word::NewLine | Word::Comment(_)))) => true,
            _ => false,
        } {
            self.source.next();
        }

        match self.source.peek() {
            None => None,
            Some(Err(_)) => None,
            Some(Ok((_, w))) => Some(w),
        }
    }
}
impl<I: Iterator<Item = TokenResult>> Iterator for Parser<I> {
    type Item = Result<Statement, ConfigurationError>;
    fn next(&mut self) -> Option<Self::Item> {
        let (position, word) = match self.source.next() {
            None => return None,
            Some(Err(e)) => return Some(Err(e)),
            Some(Ok((p, w))) => (p, w),
        };

        match word {
            Word::KeywordGenerator => Some(self.parse_generator_statement(position)),
            Word::KeywordTag => Some(self.parse_tag_statement(position)),
            Word::KeywordDir => match self.id_and_content() {
                Ok((id, blob)) => Some(Ok(Statement::Directory(position, id, blob))),
                Err(e) => Some(Err(e)),
            },
            Word::KeywordFile => match self.id_and_content() {
                Ok((id, blob)) => Some(Ok(Statement::File(position, id, blob))),
                Err(e) => Some(Err(e)),
            },
            Word::Comment(_) | Word::NewLine => self.next(),
            _ => Some(unexpected_token(position, word, "statement begin")),
        }
    }
}

#[test]
fn parse_file_statement() {
    use super::{BlobValue, Position};

    let mut parser = super::test_value(vec![
        Word::KeywordFile,
        Word::SimpleString("f".to_string()),
        Word::DollardString("The file content".to_string()),
    ]);
    assert_eq!(
        Statement::File(
            Position { line: 0, column: 1 },
            Identifier::Variable("f".to_string()),
            Blob {
                position: Position { line: 2, column: 1 },
                value: BlobValue::Literal("The file content".to_string()),
                pipes: Vec::new(),
            }
        ),
        parser.next().unwrap().unwrap()
    );
}
#[test]
fn parse_tag_statement() {
    let mut parser = super::test_value(vec![
        Word::KeywordTag,
        Word::SimpleString("simpleTag".to_string()),
    ]);

    assert_eq!(
        Statement::Tag(Position { line: 0, column: 1 }, "simpleTag".to_string()),
        parser.next().unwrap().unwrap()
    );
    assert_eq!(None, parser.next());
}
