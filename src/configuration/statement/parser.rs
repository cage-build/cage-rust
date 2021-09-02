use super::super::lexer::Word;
use super::super::ConfigurationError;
use super::Statement;

type TokenResult = Result<Word, ConfigurationError>;

enum State {
    /// Initial state
    Initial,
    /// Like Initial but if [`Word::NewLine`] is the next token else as Initial.
    WaitNewLine,
    /// Wait the tag name
    WaitTag,
}

// An iterator of [`Statement`] from an iterator of [`Word`].
pub struct Parser<I: Iterator<Item = TokenResult>> {
    source: I,
    state: Option<State>,
}
impl<I> Parser<I>
where
    I: Iterator<Item = TokenResult>,
{
    pub fn new(source: I) -> Self {
        Self {
            source,
            state: Some(State::Initial),
        }
    }
    /// Reinit the state
    fn initial_state(&mut self) {
        self.state = Some(State::Initial);
    }
}
impl<I: Iterator<Item = TokenResult>> Iterator for Parser<I> {
    type Item = Result<Statement, ConfigurationError>;
    fn next(&mut self) -> Option<Self::Item> {
        let state = match &self.state {
            Some(s) => s,
            None => return None,
        };
        let next = match self.source.next() {
            None => return None,
            Some(Err(e)) => {
                self.state = None;
                return Some(Err(e));
            }
            Some(Ok(w)) => w,
        };
        match (state, next) {
            (State::WaitNewLine, Word::NewLine) => {
                self.initial_state();
                return Some(Ok(Statement::EmptyLine));
            }
            (State::WaitNewLine, Word::Comment(c)) => {
                self.initial_state();
                return Some(Ok(Statement::Comment(c)));
            }

            (State::Initial, Word::Comment(c)) => return Some(Ok(Statement::Comment(c))),
            (State::Initial, Word::NewLine) => self.state = Some(State::WaitNewLine),
            (State::Initial | State::WaitNewLine, Word::KeywordTag) => {
                self.state = Some(State::WaitTag)
            }
            (State::Initial | State::WaitNewLine, w) => {
                panic!("Unexpected word: {:?}", w);
            }

            (State::WaitTag, Word::Variable(v)) => {
                self.state = Some(State::Initial);
                return Some(Ok(Statement::Tag(v)));
            }
            (State::WaitTag, Word::NewLine) => {}
            (State::WaitTag, Word::Comment(c)) => return Some(Ok(Statement::Comment(c))),
            (State::WaitTag, w) => {
                panic!("Unexpected word: {:?}", w);
            }
        };
        self.next()
    }
}

#[test]
fn parser_err() {
    let s = vec![Err(ConfigurationError::VersionNotFound)];
    let mut p = Parser::new(s.into_iter());
    assert_eq!(Some(Err(ConfigurationError::VersionNotFound)), p.next());
    assert_eq!(None, p.next());
}

#[test]
fn parser_tag_and_newline() {
    let s = vec![
        Ok(Word::NewLine),
        Ok(Word::KeywordTag),
        Ok(Word::NewLine),
        Ok(Word::Variable("foo".to_string())),
        Ok(Word::NewLine),
        Ok(Word::NewLine),
        Ok(Word::KeywordTag),
        Ok(Word::Variable("bar".to_string())),
        Ok(Word::NewLine),
        Ok(Word::NewLine),
    ];
    let mut p = Parser::new(s.into_iter());
    assert_eq!(Some(Ok(Statement::Tag("foo".to_string()))), p.next());
    assert_eq!(Some(Ok(Statement::EmptyLine)), p.next());
    assert_eq!(Some(Ok(Statement::Tag("bar".to_string()))), p.next());
    assert_eq!(Some(Ok(Statement::EmptyLine)), p.next());
    assert_eq!(None, p.next());
}
