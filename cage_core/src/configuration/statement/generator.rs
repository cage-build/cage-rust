use super::super::lexer::Word;
use super::super::{ConfigurationError, Position};
use super::{unexpected_token, BlobValue, Generator, Name, Parser, Statement, TokenResult};

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
        let chain = self.parse_generator_chain()?;
        Ok(Statement::Generator(position, name, chain))
    }

    /// Parse a generator chain: `{ ( "|" | ">" ) blob }`
    fn parse_generator_chain(&mut self) -> Result<Vec<Generator>, ConfigurationError> {
        let mut chain = Vec::new();
        while match self.peek() {
            Some(&Word::PipeFile | &Word::PipeDirectory) => true,
            _ => false,
        } {
            chain.push(self.parse_generator_value()?);
        }
        Ok(chain)
    }

    /// Consume element from the source iterator to parse the generator, element after like comma
    /// or declaration keyword ("file", "dir", ...) are just peeked, not consumed.
    pub fn parse_generator_value(&mut self) -> Result<Generator, ConfigurationError> {
        let (begin_position, input_is_dir) = match self.next_expected()? {
            (p, Word::PipeDirectory) => (p, true),
            (p, Word::PipeFile) => (p, false),
            (p, w) => unexpected_token(p, w, "generator pipe")?,
        };

        let (core, name) = {
            let first = self.next_expected()?;
            if self.peek() == Some(&Word::DefaultGenerator) {
                let name = match first.1 {
                    Word::QuotedString(s) | Word::DollardString(s) => s,
                    _ => unexpected_token(first.0, first.1, "generator name")?,
                };
                self.next_expected()?;
                (self.next_expected()?, Some(name))
            } else {
                (first, None)
            }
        };

        let mut g = Generator {
            position: begin_position,
            input_is_dir,
            name,
            generator: self.parse_generator_core(core)?,
            args: Vec::new(),
        };

        while match self.peek() {
            Some(Word::QuotedString(_) | Word::SimpleString(_)) => true,
            _ => false,
        } {
            let (p, w) = self.next_expected()?;
            match w {
                Word::QuotedString(s) => g.args.push((p, s)),
                Word::SimpleString(s) => g.args.push((p, s)),
                w => unexpected_token(p, w, "generator args")?,
            };
        }

        Ok(g)
    }

    /// Parse the generator core value, from a first readed value and follow if
    // it's in a parenthesis. Syntax: `name | ( blob )`
    fn parse_generator_core(
        &mut self,
        (p, w): (Position, Word),
    ) -> Result<BlobValue, ConfigurationError> {
        match w {
            Word::SimpleString(v) => Ok(BlobValue::Name(Name::Variable(v))),
            Word::QuotedString(f) => Ok(BlobValue::Name(Name::Source(f))),
            Word::DollardString(u) => Ok(BlobValue::Name(Name::Url(u))),
            Word::ParenthesisOpen => Ok(self.parse_parenthesis()?.value),
            w => unexpected_token(p, w, "generator value"),
        }
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

    let (pos, name, chain) = if let Statement::Generator(p, n, g) = parser.next().unwrap().unwrap()
    {
        (p, n, g)
    } else {
        panic!("Expected a Statement::Generator")
    };
    assert_eq!(Position { line: 0, column: 1 }, pos);
    assert_eq!("g", name.as_str());
    assert_eq!(1, chain.len());
    assert_eq!(
        Generator {
            position: Position { line: 2, column: 1 },
            input_is_dir: false,
            name: None,
            generator: BlobValue::Name(Name::Url("https://exemple.com/g.wasm".to_string())),
            args: vec![
                (Position { line: 4, column: 1 }, "arg1".to_string()),
                (Position { line: 5, column: 1 }, "arg2".to_string())
            ],
        },
        chain[0]
    );

    assert_eq!(None, parser.next());
}
#[test]
fn parse_generator_chain() {
    let mut p = super::test_value(vec![
        // first generator
        Word::PipeFile,
        Word::DollardString("https://exemple.com/generator.wasm".to_string()),
        // second generator
        Word::PipeDirectory,
        Word::DollardString("g".to_string()),
        Word::DefaultGenerator,
        Word::ParenthesisOpen,
        Word::QuotedString("generator.wasm".to_string()),
        Word::ParenthesisClose,
        Word::QuotedString("arg1".to_string()),
        Word::SimpleString("arg2".to_string()),
        // other part
        Word::Comma,
    ]);

    let chain = p.parse_generator_chain().unwrap();
    assert_eq!(2, chain.len());

    assert_eq!(
        Generator {
            position: Position { line: 0, column: 1 },
            input_is_dir: false,
            name: None,
            generator: BlobValue::Name(Name::Url("https://exemple.com/generator.wasm".to_string())),
            args: Vec::new(),
        },
        chain[0],
    );
    p.source.next();
    assert_eq!(
        Generator {
            position: Position { line: 2, column: 1 },
            input_is_dir: true,
            name: Some(String::from("g")),
            generator: BlobValue::Name(Name::Source("generator.wasm".to_string())),
            args: vec![
                (Position { line: 8, column: 1 }, "arg1".to_string()),
                (Position { line: 9, column: 1 }, "arg2".to_string())
            ],
        },
        chain[1],
    );
}
