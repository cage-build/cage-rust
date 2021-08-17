use super::char_iter::CharItem;
use super::Position;

/// Simple lexer, used as iterator to get next token.
pub struct SimpleLexer<'a> {
    chars: CharItem<'a>,
    state: SimpleLexerState,
    buff: String,
}

/// The type of token return by SimpleLexer.
#[derive(Debug, PartialEq)]
pub enum SimpleWordType {
    Word,
    String,
    Comment,
    EmptyLine,
    ErrorUncloseString,
}

/// The state of the simple lexer.
#[derive(Copy, Clone, Debug, PartialEq)]
enum SimpleLexerState {
    /// Initial state.
    Initial,
    /// Into a comment.
    Comment,
    /// Into a word.
    Word,
    /// Into a literal string.
    String,
    /// After an backslah, into a string.
    StringEscape,
}

impl<'a> SimpleLexer<'a> {
    pub fn new(config: &'a str) -> Self {
        Self {
            chars: CharItem::new(config),
            state: SimpleLexerState::Initial,
            buff: String::new(),
        }
    }
    pub fn position(&self) -> Position {
        self.chars.position()
    }
    fn word_lexer(&mut self) -> Option<SimpleWordType> {
        match (self.state, self.chars.next()) {
            (SimpleLexerState::Initial, None) => return None,
            (SimpleLexerState::Initial, Some('\n')) => return Some(SimpleWordType::EmptyLine),
            (SimpleLexerState::Initial, Some('\t' | ' ' | '\r')) => {}
            (SimpleLexerState::Initial, Some('#')) => {
                self.state = SimpleLexerState::Comment;
            }
            (SimpleLexerState::Initial, Some('"')) => {
                self.state = SimpleLexerState::String;
            }
            (SimpleLexerState::Initial, Some(c)) => {
                self.state = SimpleLexerState::Word;
                self.buff.push(c);
            }

            (SimpleLexerState::Comment, Some('\n') | None) => {
                self.state = SimpleLexerState::Initial;
                return Some(SimpleWordType::Comment);
            }
            (SimpleLexerState::Comment, Some(c)) => {
                self.buff.push(c);
            }

            (SimpleLexerState::Word, Some('#')) => {
                self.state = SimpleLexerState::Comment;
                return Some(SimpleWordType::Word);
            }
            (SimpleLexerState::Word, Some('\n' | '\t' | ' ' | '\r') | None) => {
                self.state = SimpleLexerState::Initial;
                return Some(SimpleWordType::Word);
            }
            (SimpleLexerState::Word, Some('"')) => {
                self.state = SimpleLexerState::String;
                return Some(SimpleWordType::Word);
            }
            (SimpleLexerState::Word, Some(c)) => {
                self.buff.push(c);
            }

            (SimpleLexerState::String, Some('\\')) => {
                self.state = SimpleLexerState::StringEscape;
            }
            (SimpleLexerState::String, Some('"')) => {
                self.state = SimpleLexerState::Initial;
                return Some(SimpleWordType::String);
            }
            (SimpleLexerState::String, Some(c)) => {
                self.buff.push(c);
            }
            (SimpleLexerState::StringEscape, Some(c)) => {
                self.state = SimpleLexerState::String;
                self.buff.push('\\');
                self.buff.push(c);
            }
            (SimpleLexerState::StringEscape | SimpleLexerState::String, None) => {
                self.state = SimpleLexerState::Initial;
                return Some(SimpleWordType::ErrorUncloseString);
            }
        };
        self.word_lexer()
    }
}

impl<'a> Iterator for SimpleLexer<'a> {
    type Item = (SimpleWordType, String);
    fn next(&mut self) -> Option<Self::Item> {
        self.buff.clear();
        match self.word_lexer() {
            Some(t) => Some((t, self.buff.clone())),
            None => None,
        }
    }
}

#[test]
fn test_word_lexer() {
    let s = r##"yolo = [
	{
		# A comment
		"file": $"A great literal string.
Enclose by double quote \"\".",
	},
	yoloPartent1,
	yoloPartent2,
]"##;

    let mut lexer = SimpleLexer::new(s);

    let mut t = |r: SimpleWordType, word: &str| {
        let (t, s) = lexer.next().unwrap();
        assert_eq!(r, t);
        assert_eq!(word, s);
    };
    t(SimpleWordType::Word, "yolo");
    t(SimpleWordType::Word, "=");
    t(SimpleWordType::Word, "[");
    t(SimpleWordType::Word, "{");
    t(SimpleWordType::Comment, " A comment");
    t(SimpleWordType::String, "file");
    t(SimpleWordType::Word, ":");
    t(SimpleWordType::Word, "$");
    t(
        SimpleWordType::String,
        "A great literal string.\nEnclose by double quote \\\"\\\".",
    );
    t(SimpleWordType::Word, ",");
    t(SimpleWordType::Word, "},");
    t(SimpleWordType::Word, "yoloPartent1,");
    t(SimpleWordType::Word, "yoloPartent2,");
    t(SimpleWordType::Word, "]");

    assert_eq!(None, lexer.next());
}
