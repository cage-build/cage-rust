use super::super::lexer::Word;
use super::super::{ConfigurationError, Position};
use super::{Parser, State, Statement, TokenResult};

impl<I> Parser<I>
where
    I: Iterator<Item = TokenResult>,
{
    /// Take an iterator, the [`Word::DollardString`] and [`Word::QuotedString`] must be escaped.
    pub fn new(source: I) -> Self {
        Self {
            source: source.peekable(),
            state: State::Initial,
        }
    }
    /// Reinit the state
    pub fn initial_state(&mut self) {
        self.state = State::Initial;
    }

    pub fn stake_comment(_: String) {
        unimplemented!()
    }

    /// Return the next word other than a [`Word::NewLine`] of [`Word::Comment`]
    pub fn next_expected(&mut self) -> Result<(Position, Word), ConfigurationError> {
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
        let (position, next) = match self.source.next() {
            None => return None,
            Some(Err(e)) => return Some(Err(e)),
            Some(Ok((p, w))) => (p, w),
        };
        match (self.state, next) {
            (State::WaitNewLine, Word::NewLine) => {
                self.initial_state();
                return Some(Ok(Statement::EmptyLine));
            }
            (State::WaitNewLine, Word::Comment(c)) => {
                self.initial_state();
                return Some(Ok(Statement::Comment(c)));
            }

            (State::Initial, Word::Comment(c)) => return Some(Ok(Statement::Comment(c))),
            (State::Initial, Word::NewLine) => self.state = State::WaitNewLine,
            (State::Initial | State::WaitNewLine, Word::KeywordTag) => self.state = State::WaitTag,
            (State::Initial | State::WaitNewLine, w) => {
                panic!("Unexpected word: {:?}", w);
            }

            (State::WaitTag, Word::SimpleString(v)) => {
                self.state = State::Initial;
                return Some(Ok(Statement::Tag(position, v)));
            }
            (State::WaitTag, Word::NewLine) => {}
            (State::WaitTag, Word::Comment(c)) => return Some(Ok(Statement::Comment(c))),
            (State::WaitTag, _) => {
                return Some(Err(ConfigurationError::ParserExpectedTagName(position)));
            }
        };
        self.next()
    }
}

#[test]
fn parser_err() {
    let s = vec![
        Err(ConfigurationError::VersionNotFound),
        Ok((Position::ZERO, Word::NewLine)),
        Ok((Position::ZERO, Word::NewLine)),
    ];
    let mut p = Parser::new(s.into_iter());
    assert_eq!(Some(Err(ConfigurationError::VersionNotFound)), p.next());
    assert_eq!(Some(Ok(Statement::EmptyLine)), p.next());
    assert_eq!(None, p.next());
}
#[test]
fn parser_tag_and_newline() {
    let pos_foo = Position {
        line: 17,
        column: 42,
    };
    let pos_bar = Position {
        line: 56,
        column: 124,
    };
    let s = vec![
        Ok((Position::ZERO, Word::NewLine)),
        Ok((Position::ZERO, Word::KeywordTag)),
        Ok((Position::ZERO, Word::NewLine)),
        Ok((pos_foo, Word::SimpleString("foo".to_string()))),
        Ok((Position::ZERO, Word::NewLine)),
        Ok((Position::ZERO, Word::NewLine)),
        Ok((Position::ZERO, Word::KeywordTag)),
        Ok((pos_bar, Word::SimpleString("bar".to_string()))),
        Ok((Position::ZERO, Word::NewLine)),
        Ok((Position::ZERO, Word::NewLine)),
    ];
    let mut p = Parser::new(s.into_iter());
    assert_eq!(
        Some(Ok(Statement::Tag(pos_foo, "foo".to_string()))),
        p.next()
    );
    assert_eq!(Some(Ok(Statement::EmptyLine)), p.next());
    assert_eq!(
        Some(Ok(Statement::Tag(pos_bar, "bar".to_string()))),
        p.next()
    );
    assert_eq!(Some(Ok(Statement::EmptyLine)), p.next());
    assert_eq!(None, p.next());
}
