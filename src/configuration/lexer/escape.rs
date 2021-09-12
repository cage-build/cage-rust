use super::super::{ConfigurationError, Position};

/// Escape the string follow the cage specification.
pub fn escape(p: Position, mut s: String) -> Result<String, ConfigurationError> {
    if s.find('\\').is_none() {
        return Ok(s);
    }

    let mut out = String::with_capacity(s.len());
    let mut escaping = false;
    for (i, c) in s.chars().enumerate() {
        match (escaping, c) {
            (false, '\\') => escaping = true,
            (false, c) => out.push(c),
            (true, 'n') => {
                out.push('\n');
                escaping = false;
            }
            (true, 'r') => {
                out.push('\r');
                escaping = false;
            }
            (true, 't') => {
                out.push('\t');
                escaping = false;
            }
            (true, '\\') => {
                out.push('\\');
                escaping = false;
            }
            (true, '0') => {
                out.push('\0');
                escaping = false;
            }
            (true, '\'') => {
                out.push('\'');
                escaping = false;
            }
            (true, '"') => {
                out.push('"');
                escaping = false;
            }
            (true, _) => {
                s.truncate(i);
                return Err(ConfigurationError::UnescapeFail(p, c, s));
            }
        }
    }
    Ok(out)
}

#[test]
fn test_escape() {
    let compare = |expected: &str, input: &str| {
        assert_eq!(
            expected,
            escape(Position::ZERO, input.to_string()).unwrap().as_str()
        );
    };

    compare("Hello World!", "Hello World!");
    compare("a\nb", "a\\nb");
    compare("a\rb", "a\\rb");
    compare("a\tb", "a\\tb");
    compare("a\\b", "a\\\\b");
    compare("a\0b", "a\\0b");
    compare("a'b", "a\\'b");
    compare("a\"b", "a\\\"b");
    compare("a\nb\nc\nd", "a\\nb\\nc\\nd");
}
