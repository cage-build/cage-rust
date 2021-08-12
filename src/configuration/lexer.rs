#[derive(Debug)]
pub enum Word {
    Token(String),
    Url(String),
    Colon,
    DefinitionSign,
    Coma,

    DirectoryCompositionBegin,
    DirectoryCompositionEnd,
    DirectoryConcatenateBegin,
    DirectoryConcatenateEnd,

    PipeDirectory2Directory,
    PipeDirectory2File,
    PipeFile2Directory,
    PipeFile2File,

    // for format
    WhiteSpace(String),
}

// mettre numéro de ligne et de caractère
pub fn lexer(_src: &str) -> impl Iterator<Item = Word> {
    (0..1).map(|_| Word::DirectoryCompositionBegin)
}

/*
| Entré      | Sortie     | Opérateur |
| :--------- | :--------- | :-------: |
| répertoire | répertoire |   `>>`    |
| répertoire | fichier    |   `>|`    |
| fichier    | répertoire |   `|>`    |
| fichier    | fichier    |   `||`    |

composition: {}
concatenate: []

??


*/

#[derive(Debug, PartialEq)]
enum GeneralWordType {
    String,
    Comment,
    Word,
    EmptyLine,
}

// TODO: rename to LexerState
#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Begin,
    Comment,
    Word,
    // String,
}

// TODO: line and char index
// create an iterator that

fn word_lexer(
    chars: &mut impl Iterator<Item = char>,
    buff: &mut String,
    state: &mut State,
) -> Option<GeneralWordType> {
    match (*state, chars.next()) {
        (State::Begin, None) => None,
        (State::Begin, Some('\n')) => Some(GeneralWordType::EmptyLine),
        (State::Begin, Some('\t' | ' ' | '\r')) => word_lexer(chars, buff, state),
        (State::Begin, Some('#')) => {
            *state = State::Comment;
            buff.clear();
            word_lexer(chars, buff, state)
        }
        (State::Begin, Some(c)) => {
            *state = State::Word;
            buff.clear();
            buff.push(c);
            word_lexer(chars, buff, state)
        }

        (State::Comment, Some('\n') | None) => {
            *state = State::Begin;
            Some(GeneralWordType::Comment)
        }
        (State::Comment, Some(c)) => {
            buff.push(c);
            word_lexer(chars, buff, state)
        }

        (State::Word, Some('#')) => {
            *state = State::Comment;
            Some(GeneralWordType::Word)
        }
        (State::Word, Some('\n' | '\t' | ' ' | '\r') | None) => {
            *state = State::Begin;
            Some(GeneralWordType::Word)
        }
        (State::Word, Some(c)) => {
            buff.push(c);
            word_lexer(chars, buff, state)
        }
    }
}

#[test]
fn test_word_lexer() {
    let mut buff = String::new();
    let mut state = State::Word;
    let mut chars = r#"yolo = [
	{
		# A comment
		# "sdfb":
	},
	yoloPartent1,
	yoloPartent2,
]"#
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
    t(GeneralWordType::Comment, " \"sdfb\":");
    t(GeneralWordType::Word, "},");
    t(GeneralWordType::Word, "yoloPartent1,");
    t(GeneralWordType::Word, "yoloPartent2,");
    t(GeneralWordType::Word, "]");

    assert_eq!(None, word_lexer(&mut chars, &mut buff, &mut state));
}
