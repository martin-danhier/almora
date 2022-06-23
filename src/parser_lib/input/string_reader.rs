use std::str::Chars;

use crate::parser_lib::MatchToken;

pub struct StringReader<'a> {
    string: &'a str,
    pos: usize,
}

impl<'a> StringReader<'a> {
    pub fn new(string: &'a str) -> Self {
        Self { string, pos: 0 }
    }
}

impl<'a> MatchToken for StringReader<'a> {
    fn match_str(&mut self, string: &'static str) -> bool {
        if self.string[self.pos..].starts_with(string) {
            // It matches
            self.pos += string.len();
            true
        } else {
            false
        }
    }

    fn match_identifier(&mut self) -> Option<String> {
        if self.pos >= self.string.len() {
            return None;
        }

        let mut ident = String::new();
        let mut chars = self.string[self.pos..].chars();

        // Match [a-zA-Z_][a-zA-Z0-9_]*
        while let Some(c) = chars.next() {
            if c.is_ascii_alphabetic() || c == '_' || (!ident.is_empty() && c.is_digit(10)) {
                ident.push(c);
            } else {
                break;
            }
        }

        match ident.is_empty() {
            true => None,
            false => {
                self.pos += ident.len();
                Some(ident)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_reader() {
        let mut reader = StringReader::new("😎 hello");

        assert!(reader.match_str("😎"));
        assert!(!reader.match_str("world"));
        assert!(reader.match_str(" "));
        assert_eq!(reader.match_identifier(), Some("hello".to_string()));
        assert_eq!(reader.match_identifier(), None);
    }
}
