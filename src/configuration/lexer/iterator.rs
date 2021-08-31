use super::{CharItem, Lexer, LexerError, Position, Word};

/// The state of the lexer.
#[derive(Debug, Copy, Clone)]
pub enum State {
    /// For initial, or
    Initial,
    /// Inside a variable or a keyword.
    Word,
    /// Inside a comment
    Comment,
    /// Inside a file path
    File,
    /// Inside a file path, after a backslash
    FileEscape,
    /// Begin of one question mark.
    QuestionMark,
    /// Just after a dollar
    Dollar,
    /// inside a system variable
    System,
    /// Inside a literal string
    String,
    /// Inside a literal string, after the backslah
    StringEscape,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Position, Word);
    fn next(&mut self) -> Option<Self::Item> {
        if self.error.is_some() {
            return None;
        } else if let Some(r) = self.comming.take() {
            return Some((self.chars.position(), r));
        }
        self.buff.clear();
        let p = self.chars.position();
        self.word_lexer().map(|w| (p, w))
    }
}

impl<'a> Lexer<'a> {
    /// Create a new Lexer for the config.
    pub fn new(config: &'a str) -> Self {
        Self {
            chars: CharItem::new(config),
            state: State::Initial,
            buff: String::new(),
            comming: None,
            error: None,
        }
    }

    /// Get a Word from self.buffer, return a keyword or a variable. Always `Some(Ok(_))`.
    fn type_word(&self) -> Option<Word> {
        Some(match &self.buff[..] {
            "dir" => Word::KeywordDir,
            "file" => Word::KeywordFile,
            "tag" => Word::KeywordTag,
            _ => Word::Variable(self.buff.clone()),
        })
    }

    /// Get a Word from self.buffer. Always `Some()`.
    fn type_system(&mut self) -> Option<Word> {
        match &self.buff[..] {
            "pkg" => Some(Word::SystemPackage),
            "run" => Some(Word::SystemRun),
            "test" => Some(Word::SystemTest),
            _ => {
                self.set_err(LexerError::UnknowSystem(self.buff.clone()));
                None
            }
        }
    }

    // Save error, and return None.
    fn set_err(&mut self, e: LexerError) {
        self.error = Some(e);
        self.state = State::Initial;
    }

    /// Take and return the error. To call at the end.
    pub fn err(self) -> Option<(Position, LexerError)> {
        let p = self.chars.position();
        self.error.map(|e| (p, e))
    }

    /// Fill self.buff and self.comming
    fn word_lexer(&mut self) -> Option<Word> {
        match (self.state, self.chars.next()) {
            (State::Initial, None) => return None,
            (State::Initial, Some(' ' | '\t')) => {}
            (State::Initial, Some('\n')) => return Some(Word::NewLine),
            (State::Initial, Some('[')) => return Some(Word::DirectoryConcatOpen),
            (State::Initial, Some(']')) => return Some(Word::DirectoryConcatClose),
            (State::Initial, Some('{')) => return Some(Word::DirectoryComposeOpen),
            (State::Initial, Some('}')) => return Some(Word::DirectoryComposeClose),
            (State::Initial, Some(':')) => return Some(Word::Colon),
            (State::Initial, Some(',')) => return Some(Word::Comma),
            (State::Initial, Some('|')) => return Some(Word::PipeFile),
            (State::Initial, Some('>')) => return Some(Word::PipeDirectory),
            (State::Initial, Some('#')) => self.state = State::Comment,
            (State::Initial, Some('"')) => self.state = State::File,
            (State::Initial, Some('?')) => self.state = State::QuestionMark,
            (State::Initial, Some('$')) => self.state = State::Dollar,

            (State::QuestionMark, Some('?')) => {
                self.state = State::Initial;
                return Some(Word::DefaultGenerator);
            }
            (State::QuestionMark, _) => {
                self.set_err(LexerError::HalfDefaultGenerator);
                return None;
            }

            (State::Comment, None | Some('\n')) => {
                self.state = State::Initial;
                return Some(Word::Comment(self.buff.clone()));
            }
            (State::Comment, Some(c)) => self.buff.push(c),

            (State::File, Some('"')) => {
                self.state = State::Initial;
                return Some(Word::File(self.buff.clone()));
            }
            (State::File, Some('\\')) => self.state = State::FileEscape,
            (State::File, Some(c)) => self.buff.push(c),
            (State::File | State::FileEscape, None) => {
                self.set_err(LexerError::StringWithoutEnd);
                return None;
            }
            (State::FileEscape, Some(c)) => {
                self.buff.push('\\');
                self.buff.push(c);
                self.state = State::File;
            }

            (State::Word, Some('#')) => {
                self.state = State::Comment;
                return self.type_word();
            }
            (State::Word, Some('"')) => {
                self.state = State::File;
                return self.type_word();
            }
            (State::Word, Some('[')) => {
                self.state = State::Initial;
                self.comming = Some(Word::DirectoryConcatOpen);
                return self.type_word();
            }
            (State::Word, Some(']')) => {
                self.state = State::Initial;
                self.comming = Some(Word::DirectoryConcatClose);
                return self.type_word();
            }
            (State::Word, Some('{')) => {
                self.state = State::Initial;
                self.comming = Some(Word::DirectoryComposeOpen);
                return self.type_word();
            }
            (State::Word, Some('}')) => {
                self.state = State::Initial;
                self.comming = Some(Word::DirectoryComposeClose);
                return self.type_word();
            }
            (State::Word, Some(':')) => {
                self.state = State::Initial;
                self.comming = Some(Word::Colon);
                return self.type_word();
            }
            (State::Word, Some(',')) => {
                self.state = State::Initial;
                self.comming = Some(Word::Comma);
                return self.type_word();
            }
            (State::Word, Some('|')) => {
                self.state = State::Initial;
                self.comming = Some(Word::PipeFile);
                return self.type_word();
            }
            (State::Word, Some('>')) => {
                self.state = State::Initial;
                self.comming = Some(Word::PipeDirectory);
                return self.type_word();
            }
            (State::Word, Some('?')) => {
                self.state = State::QuestionMark;
                return self.type_word();
            }
            (State::Word, None | Some(' ' | '\t')) => {
                self.state = State::Initial;
                return self.type_word();
            }
            (State::Word, Some('\n')) => {
                self.state = State::Initial;
                self.comming = Some(Word::NewLine);
                return self.type_word();
            }
            (State::Word, Some('$')) => {
                self.state = State::Dollar;
                return self.type_word();
            }
            (State::Initial | State::Word, Some(c)) if c.is_alphanumeric() => {
                self.state = State::Word;
                self.buff.push(c);
            }

            (State::System, Some('#')) => {
                self.state = State::Comment;
                return self.type_system();
            }
            (State::System, Some('"')) => {
                self.state = State::File;
                return self.type_system();
            }
            (State::System, Some('[')) => {
                self.state = State::Initial;
                self.comming = Some(Word::DirectoryConcatOpen);
                return self.type_system();
            }
            (State::System, Some(']')) => {
                self.state = State::Initial;
                self.comming = Some(Word::DirectoryConcatClose);
                return self.type_system();
            }
            (State::System, Some('{')) => {
                self.state = State::Initial;
                self.comming = Some(Word::DirectoryComposeOpen);
                return self.type_system();
            }
            (State::System, Some('}')) => {
                self.state = State::Initial;
                self.comming = Some(Word::DirectoryComposeClose);
                return self.type_system();
            }
            (State::System, Some(':')) => {
                self.state = State::Initial;
                self.comming = Some(Word::Colon);
                return self.type_system();
            }
            (State::System, Some(',')) => {
                self.state = State::Initial;
                self.comming = Some(Word::Comma);
                return self.type_system();
            }
            (State::System, Some('|')) => {
                self.state = State::Initial;
                self.comming = Some(Word::PipeFile);
                return self.type_system();
            }
            (State::System, Some('>')) => {
                self.state = State::Initial;
                self.comming = Some(Word::PipeDirectory);
                return self.type_system();
            }
            (State::System, Some('?')) => {
                self.state = State::QuestionMark;
                return self.type_system();
            }
            (State::System, None | Some(' ' | '\t')) => {
                self.state = State::Initial;
                return self.type_system();
            }
            (State::System, Some('\n')) => {
                self.state = State::Initial;
                self.comming = Some(Word::NewLine);
                return self.type_system();
            }
            (State::System, Some('$')) => {
                self.state = State::Dollar;
                return self.type_system();
            }
            (State::System, Some(c)) if c.is_alphanumeric() => {
                self.state = State::System;
                self.buff.push(c);
            }

            (State::Dollar, Some('$')) => {
                self.set_err(LexerError::DoubleDollard);
                return None;
            }
            (State::Dollar, Some('"')) => self.state = State::String,
            (State::Dollar, Some(c)) if c.is_alphanumeric() => {
                self.state = State::System;
                self.buff.push(c);
            }

            (State::Dollar, None) => {
                self.set_err(LexerError::DollardAtEOF);
                return None;
            }

            (State::String, Some('"')) => {
                self.state = State::Initial;
                return Some(Word::String(self.buff.clone()));
            }
            (State::String, Some('\\')) => self.state = State::StringEscape,
            (State::String, Some(c)) => self.buff.push(c),
            (State::String | State::StringEscape, None) => {
                self.set_err(LexerError::StringWithoutEnd);
                return None;
            }

            (State::StringEscape, Some(c)) => {
                self.state = State::String;
                self.buff.push('\\');
                self.buff.push(c);
            }

            (State::Initial | State::Word | State::System | State::Dollar, Some(c)) => {
                self.set_err(LexerError::UnknowChar(c));
                return None;
            }
        };
        self.word_lexer()
    }
}
