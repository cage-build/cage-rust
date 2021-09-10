use super::super::lexer::Word;
use super::super::{ConfigurationError, Position};
use super::{Generator, GeneratorKind, Statement};
use std::iter::Peekable;

type TokenResult = Result<(Position, Word), ConfigurationError>;

#[derive(Debug, Copy, Clone)]
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
    source: Peekable<I>,
    state: State,
}
impl<I> Parser<I>
where
    I: Iterator<Item = TokenResult>,
{
    pub fn new(source: I) -> Self {
        Self {
            source: source.peekable(),
            state: State::Initial,
        }
    }
    /// Reinit the state
    fn initial_state(&mut self) {
        self.state = State::Initial;
    }

    fn stake_comment(_: String) {
        unimplemented!()
    }

    /// Return the next word other than a [`Word::NewLine`] of [`Word::Comment`]
    fn next_expected(&mut self) -> Result<(Position, Word), ConfigurationError> {
        match self.source.next() {
            Some(Ok((_, Word::NewLine | Word::Comment(_)))) => self.next_expected(),
            Some(Ok((p, w))) => Ok((p, w)),
            Some(Err(e)) => Err(e),
            None => Err(ConfigurationError::UnexpectedEnd),
        }
    }

    /// Peek the next token other than a [`Word::NewLine`] of [`Word::Comment`]. Return None if error.
    fn peek(&mut self) -> Option<&Word> {
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

    /// Consume element from the source iterator to parse the generator, element after like comma
    /// or declaration keyword ("file", "dir", ...) are just peeked, not consumed.
    fn parse_generator_value(&mut self) -> Result<Generator, ConfigurationError> {
        /// Take an world an return the Genrator Kind
        fn word2_generator_kind(
            (p, w): (Position, Word),
        ) -> Result<GeneratorKind, ConfigurationError> {
            match w {
                Word::Variable(v) => Ok(GeneratorKind::Variable(v)),
                Word::File(f) => Ok(GeneratorKind::File(f)),
                Word::String(u) => Ok(GeneratorKind::Url(u)),
                w => Err(ConfigurationError::ParserWrongGeneratorToken(
                    p,
                    format!("{:?}", w),
                )),
            }
        }

        let first = self.next_expected()?;
        let g: Generator = if self.peek() == Some(&Word::DefaultGenerator) {
            let name = Some(match first.1 {
                Word::File(s) | Word::String(s) => s,
                w => {
                    return Err(ConfigurationError::ParserGeneratorNameToken(
                        first.0,
                        format!("{:?}", w),
                    ));
                }
            });
            self.source.next();
            Generator {
                position: first.0,
                name,
                generator: word2_generator_kind(self.next_expected()?)?,
                args: Vec::new(),
            }
        } else {
            Generator {
                position: first.0,
                name: None,
                generator: word2_generator_kind(first)?,
                args: Vec::new(),
            }
        };

        Ok(g)
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

            (State::WaitTag, Word::Variable(v)) => {
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
fn parse_generator_value() {
    let pos_gen_1 = Position { line: 1, column: 5 };
    let pos_gen_2 = Position { line: 2, column: 6 };
    let src = vec![
        Ok((
            pos_gen_1,
            Word::String("https://exemple.com/generator.wasm".to_string()),
        )),
        Ok((Position::ZERO, Word::Comma)),
        Ok((pos_gen_2, Word::String("g".to_string()))),
        Ok((Position::ZERO, Word::NewLine)),
        Ok((Position::ZERO, Word::DefaultGenerator)),
        Ok((Position::ZERO, Word::Comment("a comment".to_string()))),
        Ok((Position::ZERO, Word::File("generator.wasm".to_string()))),
        Ok((Position::ZERO, Word::Comma)),
    ];
    let mut p = Parser::new(src.into_iter());

    assert_eq!(
        Generator {
            position: pos_gen_1,
            name: None,
            generator: GeneratorKind::Url("https://exemple.com/generator.wasm".to_string()),
            args: Vec::new(),
        },
        p.parse_generator_value().unwrap()
    );
    p.source.next();
    assert_eq!(
        Generator {
            position: pos_gen_2,
            name: Some(String::from("g")),
            generator: GeneratorKind::File("generator.wasm".to_string()),
            args: Vec::new(),
        },
        p.parse_generator_value().unwrap()
    );
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
        Ok((pos_foo, Word::Variable("foo".to_string()))),
        Ok((Position::ZERO, Word::NewLine)),
        Ok((Position::ZERO, Word::NewLine)),
        Ok((Position::ZERO, Word::KeywordTag)),
        Ok((pos_bar, Word::Variable("bar".to_string()))),
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
