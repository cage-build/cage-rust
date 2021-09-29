use super::{CharItem, Lexer, LexerError, Word};

impl<'a> Lexer<'a> {
    /// Create a new Lexer for the config.
    pub fn new(config: &'a str) -> Self {
        Self {
            chars: CharItem::new(config),
            buff: String::new(),
            comming_char: None,
        }
    }

    pub fn word_lexer(&mut self) -> Option<Result<Word, LexerError>> {
        Some(
            match self.comming_char.take().or_else(|| self.chars.next())? {
                '\t' | ' ' => return self.word_lexer(),
                '[' => Ok(Word::DirectoryConcatOpen),
                ']' => Ok(Word::DirectoryConcatClose),
                '(' => Ok(Word::ParenthesisOpen),
                ')' => Ok(Word::ParenthesisClose),
                '{' => Ok(Word::DirectoryComposeOpen),
                '}' => Ok(Word::DirectoryComposeClose),
                ':' => Ok(Word::Colon),
                ',' => Ok(Word::Comma),
                '|' => Ok(Word::PipeFile),
                '>' => Ok(Word::PipeDirectory),
                '\n' => Ok(Word::NewLine),

                '?' => match self.chars.next() {
                    Some('?') => Ok(Word::DefaultGenerator),
                    Some(_) | None => Err(LexerError::HalfDefaultGenerator),
                },
                '$' => match self.chars.next() {
                    None => Err(LexerError::DollardAtEOF),
                    Some(s) if Self::is_special(s) => Err(LexerError::UnknowChar(s)),
                    Some('"') => self.read_string(true),
                    Some(c) => {
                        self.buff.push(c);
                        self.read_var(true)
                    }
                },
                '"' => self.read_string(false),
                '#' => Ok(self.read_comment()),
                c => {
                    self.buff.push(c);
                    self.read_var(false)
                }
            },
        )
    }

    /// We have encouter a dollar, read the next token.
    fn read_dollar_next(&mut self) -> Result<Word, LexerError> {
        match self.chars.next() {
            None => Err(LexerError::DollardAtEOF),
            Some('$') => Err(LexerError::DoubleDollard),
            Some('"') => self.read_string(true),
            Some(c) if c.is_alphanumeric() => {
                self.buff.push(c);
                self.read_var(true)
            }
            Some(c) => Err(LexerError::UnknowChar(c)),
        }
    }

    /// Read the (system) variable or keyword. Maybe preceded by a dollard.
    fn read_var(&mut self, dollar_begin: bool) -> Result<Word, LexerError> {
        loop {
            match self.chars.next() {
                None => break,
                Some(s) if Self::is_special(s) => {
                    self.comming_char = Some(s);
                    break;
                }
                Some(c) => self.buff.push(c),
            }
        }
        match (dollar_begin, &self.buff[..]) {
            (true, "pkg") => Ok(Word::SystemPackage),
            (true, "run") => Ok(Word::SystemRun),
            (true, "test") => Ok(Word::SystemTest),
            (true, _) => Err(LexerError::UnknowSystem(self.buff.clone())),
            (false, "dir") => Ok(Word::KeywordDir),
            (false, "file") => Ok(Word::KeywordFile),
            (false, "tag") => Ok(Word::KeywordTag),
            (false, "gen") => Ok(Word::KeywordGenerator),
            (false, s) => Ok(Word::SimpleString(s.to_string())),
        }
    }

    /// Read all the string token.
    fn read_string(&mut self, dollar_begin: bool) -> Result<Word, LexerError> {
        loop {
            match self.chars.next() {
                Some('"') => break,
                Some('\\') => {
                    self.buff.push('\\');
                    match self.chars.next() {
                        Some(c) => self.buff.push(c),
                        None => return Err(LexerError::StringWithoutEnd),
                    }
                }
                Some(c) => self.buff.push(c),
                None => return Err(LexerError::StringWithoutEnd),
            }
        }
        Ok(match dollar_begin {
            true => Word::DollardString(self.buff.clone()),
            false => Word::QuotedString(self.buff.clone()),
        })
    }

    /// Read all comment.
    fn read_comment(&mut self) -> Word {
        loop {
            match self.chars.next() {
                Some('\n') | None => break,
                Some(c) => self.buff.push(c),
            };
        }
        Word::Comment(self.buff.clone())
    }

    /// This char can not be in a variable
    fn is_special(c: char) -> bool {
        match c {
            '$' | ' ' | '?' | '\t' | '\n' | '[' | ']' | '(' | ')' | '{' | '}' | ':' | ',' | '|'
            | '>' | '#' => true,
            _ => false,
        }
    }
}
