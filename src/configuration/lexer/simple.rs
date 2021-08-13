#[derive(Debug, PartialEq)]
enum GeneralWordType {
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

// TODO: line and char index
// create an iterator that

fn word_lexer(
    chars: &mut impl Iterator<Item = char>,
    buff: &mut String,
    state: &mut SimpleLexerState,
) -> Option<GeneralWordType> {
    match (*state, chars.next()) {
        (SimpleLexerState::Initial, None) => return None,
        (SimpleLexerState::Initial, Some('\n')) => return Some(GeneralWordType::EmptyLine),
        (SimpleLexerState::Initial, Some('\t' | ' ' | '\r')) => {}
        (SimpleLexerState::Initial, Some('#')) => {
            *state = SimpleLexerState::Comment;
        }
        (SimpleLexerState::Initial, Some('"')) => {
            *state = SimpleLexerState::String;
            buff.clear();
        }
        (SimpleLexerState::Initial, Some(c)) => {
            *state = SimpleLexerState::Word;
            buff.push(c);
        }

        (SimpleLexerState::Comment, Some('\n') | None) => {
            *state = SimpleLexerState::Initial;
            return Some(GeneralWordType::Comment);
        }
        (SimpleLexerState::Comment, Some(c)) => {
            buff.push(c);
        }

        (SimpleLexerState::Word, Some('#')) => {
            *state = SimpleLexerState::Comment;
            return Some(GeneralWordType::Word);
        }
        (SimpleLexerState::Word, Some('\n' | '\t' | ' ' | '\r') | None) => {
            *state = SimpleLexerState::Initial;
            return Some(GeneralWordType::Word);
        }
        (SimpleLexerState::Word, Some('"')) => {
            *state = SimpleLexerState::String;
            return Some(GeneralWordType::Word);
        }
        (SimpleLexerState::Word, Some(c)) => {
            buff.push(c);
        }

        (SimpleLexerState::String, Some('\\')) => {
            *state = SimpleLexerState::StringEscape;
        }
        (SimpleLexerState::String, Some('"')) => {
            *state = SimpleLexerState::Initial;
            return Some(GeneralWordType::String);
        }
        (SimpleLexerState::String, Some(c)) => {
            buff.push(c);
        }
        (SimpleLexerState::StringEscape, Some(c)) => {
            *state = SimpleLexerState::String;
            buff.push('\\');
            buff.push(c);
        }
        (SimpleLexerState::StringEscape | SimpleLexerState::String, None) => {
            *state = SimpleLexerState::Initial;
            return Some(GeneralWordType::ErrorUncloseString);
        }
    };
    word_lexer(chars, buff, state)
}

#[test]
fn test_word_lexer() {
    let mut buff = String::new();
    let mut state = SimpleLexerState::Initial;
    let mut chars = r##"yolo = [
	{
		# A comment
		"file": $"A great literal string.
Enclose by double quote \"\".",
	},
	yoloPartent1,
	yoloPartent2,
]"##
    .chars();

    let mut t = |r: GeneralWordType, word: &str| {
        assert_eq!(Some(r), word_lexer(&mut chars, &mut buff, &mut state));
        assert_eq!(word, &buff);
        buff.clear();
    };
    t(GeneralWordType::Word, "yolo");
    t(GeneralWordType::Word, "=");
    t(GeneralWordType::Word, "[");
    t(GeneralWordType::Word, "{");
    t(GeneralWordType::Comment, " A comment");
    t(GeneralWordType::String, "file");
    t(GeneralWordType::Word, ":");
    t(GeneralWordType::Word, "$");
    t(
        GeneralWordType::String,
        "A great literal string.\nEnclose by double quote \\\"\\\".",
    );
    t(GeneralWordType::Word, ",");
    t(GeneralWordType::Word, "},");
    t(GeneralWordType::Word, "yoloPartent1,");
    t(GeneralWordType::Word, "yoloPartent2,");
    t(GeneralWordType::Word, "]");

    assert_eq!(None, word_lexer(&mut chars, &mut buff, &mut state));
}
