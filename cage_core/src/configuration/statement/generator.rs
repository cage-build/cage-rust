use super::super::lexer::Word;
use super::super::{ConfigurationError, Position};
use super::{Generator, GeneratorValue, Parser, TokenResult};
use std::convert::TryFrom;

impl<I> Parser<I>
where
    I: Iterator<Item = TokenResult>,
{
    /// Consume element from the source iterator to parse the generator, element after like comma
    /// or declaration keyword ("file", "dir", ...) are just peeked, not consumed.
    fn parse_generator_value(&mut self) -> Result<Generator, ConfigurationError> {
        impl TryFrom<(Position, Word)> for GeneratorValue {
            type Error = ConfigurationError;
            fn try_from((p, w): (Position, Word)) -> Result<Self, Self::Error> {
                match w {
                    Word::SimpleString(v) => Ok(GeneratorValue::Variable(v)),
                    Word::QuotedString(f) => Ok(GeneratorValue::File(f)),
                    Word::DollardString(u) => Ok(GeneratorValue::Url(u)),
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
                position: first.0,
                name,
                generator: GeneratorValue::try_from(self.next_expected()?)?,
                args: Vec::new(),
            }
        } else {
            Generator {
                position: first.0,
                name: None,
                generator: GeneratorValue::try_from(first)?,
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
fn parse_generator_value() {
    let pos_gen_1 = Position { line: 1, column: 5 };
    let pos_gen_2 = Position { line: 2, column: 6 };
    let pos_arg_1 = Position {
        line: 3,
        column: 20,
    };
    let pos_arg_2 = Position {
        line: 3,
        column: 30,
    };
    let src = vec![
        Ok((
            pos_gen_1,
            Word::DollardString("https://exemple.com/generator.wasm".to_string()),
        )),
        Ok((Position::ZERO, Word::Comma)),
        Ok((pos_gen_2, Word::DollardString("g".to_string()))),
        Ok((Position::ZERO, Word::NewLine)),
        Ok((Position::ZERO, Word::DefaultGenerator)),
        Ok((Position::ZERO, Word::Comment("a comment".to_string()))),
        Ok((
            Position::ZERO,
            Word::QuotedString("generator.wasm".to_string()),
        )),
        Ok((pos_arg_1, Word::QuotedString("arg1".to_string()))),
        Ok((pos_arg_2, Word::SimpleString("arg2".to_string()))),
        Ok((Position::ZERO, Word::Comma)),
    ];
    let mut p = Parser::new(src.into_iter());

    assert_eq!(
        Generator {
            position: pos_gen_1,
            name: None,
            generator: GeneratorValue::Url("https://exemple.com/generator.wasm".to_string()),
            args: Vec::new(),
        },
        p.parse_generator_value().unwrap()
    );
    p.source.next();
    assert_eq!(
        Generator {
            position: pos_gen_2,
            name: Some(String::from("g")),
            generator: GeneratorValue::File("generator.wasm".to_string()),
            args: vec![
                (pos_arg_1, "arg1".to_string()),
                (pos_arg_2, "arg2".to_string())
            ],
        },
        p.parse_generator_value().unwrap()
    );
}
