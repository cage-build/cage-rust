use core::str::Chars;

pub struct CharItem<'a> {
    chars_iter: Chars<'a>,
    line: usize,
    column: usize,
}

#[derive(Debug, Copy, Clone, std::cmp::PartialEq)]
pub struct Position {
    line: usize,
    column: usize,
}

impl<'a> CharItem<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            chars_iter: s.chars(),
            line: 1,
            column: 1,
        }
    }
    pub fn position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
        }
    }
}

impl<'a> Iterator for CharItem<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.chars_iter.next();
        match next {
            Some('\n') => {
                self.line += 1;
                self.column = 1;
            }
            Some(_) => {
                self.column += 1;
            }
            None => {}
        };
        next
    }
}

#[test]
fn test_char_iter() {
    let mut iter = CharItem::new("Hel\nlo");
    assert_eq!(Some('H'), iter.next());
    assert_eq!(Position { line: 1, column: 2 }, iter.position());
    assert_eq!(Some('e'), iter.next());
    assert_eq!(Position { line: 1, column: 3 }, iter.position());
    assert_eq!(Some('l'), iter.next());
    assert_eq!(Position { line: 1, column: 4 }, iter.position());
    assert_eq!(Some('\n'), iter.next());
    assert_eq!(Position { line: 2, column: 1 }, iter.position());
    assert_eq!(Some('l'), iter.next());
    assert_eq!(Position { line: 2, column: 2 }, iter.position());
    assert_eq!(Some('o'), iter.next());
    assert_eq!(Position { line: 2, column: 3 }, iter.position());
}
