use super::super::lexer::Word;
use super::super::{ConfigurationError, Position};
use super::{unexpected_token, Generator, Name, Parser, Statement, TokenResult};
use std::convert::TryFrom;

impl<I> Parser<I>
where
    I: Iterator<Item = TokenResult>,
{
    /// Parse the name and the value of the generator statement.
    /// The keyword already readed supplies the position of this statement given in arguments.
    pub fn parse_generator_statement(
        &mut self,
        position: Position,
    ) -> Result<Statement, ConfigurationError> {
        let name = self.get_statment_name()?;
        let value = self.parse_generator_value()?;
        Ok(Statement::Generator(position, name, value))
    }

    /// Consume element from the source iterator to parse the generator, element after like comma
    /// or declaration keyword ("file", "dir", ...) are just peeked, not consumed.
    pub fn parse_generator_value(&mut self) -> Result<Generator, ConfigurationError> {
        let (begin_position, input_is_dir) = match self.next_expected()? {
            (p, Word::PipeDirectory) => (p, true),
            (p, Word::PipeFile) => (p, false),
            (p, w) => unexpected_token(p, w, "generator pipe")?,
        };

        impl TryFrom<(Position, Word)> for Name {
            type Error = ConfigurationError;
            fn try_from((p, w): (Position, Word)) -> Result<Self, Self::Error> {
                match w {
                    Word::SimpleString(v) => Ok(Name::Variable(v)),
                    Word::QuotedString(f) => Ok(Name::Source(f)),
                    Word::DollardString(u) => Ok(Name::Url(u)),
                    w => Err(ConfigurationError::ParserWrongGeneratorToken(
                        p,
                        format!("{:?}", w),
                    )),
                }
            }
        }

        let first = self.next_expected()?;
        let mut g: Generator = if self.peek() == Some(&Word::DefaultGenerator) {
            let name = Some(match first.1 {
                Word::QuotedString(s) | Word::DollardString(s) => s,
                w => {
                    return Err(ConfigurationError::ParserGeneratorNameToken(
                        first.0,
                        format!("{:?}", w),
                    ));
                }
            });
            self.source.next();
            Generator {
                position: begin_position,
                input_is_dir,
                name,
                generator: Name::try_from(self.next_expected()?)?,
                args: Vec::new(),
            }
        } else {
            Generator {
                position: begin_position,
                input_is_dir,
                name: None,
                generator: Name::try_from(first)?,
                args: Vec::new(),
            }
        };

        while match self.peek() {
            Some(Word::QuotedString(_) | Word::SimpleString(_)) => true,
            _ => false,
        } {
            let (p, w) = self.next_expected()?;
            match w {
                Word::QuotedString(s) => g.args.push((p, s)),
                Word::SimpleString(s) => g.args.push((p, s)),
                w => {
                    return Err(ConfigurationError::UnexpectedToken(
                        p,
                        format!("{:?}", w),
                        "generator args",
                    ))
                }
            };
        }

        Ok(g)
    }
}

#[test]
fn parse_generator_statement() {
    let mut parser = super::test_value(vec![
        Word::KeywordGenerator,
        Word::SimpleString("g".to_string()),
        Word::PipeFile,
        Word::DollardString("https://exemple.com/g.wasm".to_string()),
        Word::QuotedString("arg1".to_string()),
        Word::QuotedString("arg2".to_string()),
    ]);

    let (pos, name, gen) = if let Statement::Generator(p, n, g) = parser.next().unwrap().unwrap() {
        (p, n, g)
    } else {
        panic!("Expected a Statement::Generator")
    };
    assert_eq!(Position { line: 0, column: 1 }, pos);
    assert_eq!("g", name.as_str());
    assert_eq!(
        Generator {
            position: Position { line: 2, column: 1 },
            input_is_dir: false,
            name: None,
            generator: Name::Url("https://exemple.com/g.wasm".to_string()),
            args: vec![
                (Position { line: 4, column: 1 }, "arg1".to_string()),
                (Position { line: 5, column: 1 }, "arg2".to_string())
            ],
        },
        gen
    );

    assert_eq!(None, parser.next());
}

#[test]
fn parse_generator_value() {
    let mut p = super::test_value(vec![
        Word::PipeFile,
        Word::DollardString("https://exemple.com/generator.wasm".to_string()),
        Word::Comma,
        Word::PipeDirectory,
        Word::DollardString("g".to_string()),
        Word::NewLine,
        Word::DefaultGenerator,
        Word::Comment("a comment".to_string()),
        Word::QuotedString("generator.wasm".to_string()),
        Word::QuotedString("arg1".to_string()),
        Word::SimpleString("arg2".to_string()),
        Word::Comma,
    ]);

    assert_eq!(
        Generator {
            position: Position { line: 0, column: 1 },
            input_is_dir: false,
            name: None,
            generator: Name::Url("https://exemple.com/generator.wasm".to_string()),
            args: Vec::new(),
        },
        p.parse_generator_value().unwrap()
    );
    p.source.next();
    assert_eq!(
        Generator {
            position: Position { line: 3, column: 1 },
            input_is_dir: true,
            name: Some(String::from("g")),
            generator: Name::Source("generator.wasm".to_string()),
            args: vec![
                (Position { line: 9, column: 1 }, "arg1".to_string()),
                (
                    Position {
                        line: 10,
                        column: 1
                    },
                    "arg2".to_string()
                )
            ],
        },
        p.parse_generator_value().unwrap()
    );
}
