use std::str::Chars;

use crate::utils::Peek;

pub struct StringReader<'a> {
    chars: Chars<'a>,
}

impl<'a> StringReader<'a> {
    pub fn new(string: &'a str) -> Self {
        Self {
            chars: string.chars(),
        }
    }
}

impl<'a> Iterator for StringReader<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next()
    }
}

impl<'a> Peek for StringReader<'a> {
    fn peek(&mut self) -> Option<Self::Item> {
        self.chars.clone().next()
    }

    fn peek_nth(&mut self, n: usize) -> Option<Self::Item> {
        self.chars.clone().nth(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_reader() {
        let mut reader = StringReader::new("😎 hello");

        assert_eq!(reader.peek(), Some('😎'));
        assert_eq!(reader.peek_nth(3), Some('e'));
        assert_eq!(reader.next(), Some('😎'));

        assert_eq!(reader.peek(), Some(' '));
        assert_eq!(reader.next(), Some(' '));

        assert_eq!(reader.peek(), Some('h'));
        assert_eq!(reader.next(), Some('h'));

        assert_eq!(reader.peek(), Some('e'));
        assert_eq!(reader.next(), Some('e'));

        assert_eq!(reader.peek(), Some('l'));
        assert_eq!(reader.next(), Some('l'));

        assert_eq!(reader.peek(), Some('l'));
        assert_eq!(reader.peek_nth(3), None);
        assert_eq!(reader.next(), Some('l'));

        assert_eq!(reader.peek(), Some('o'));
        assert_eq!(reader.next(), Some('o'));

        assert_eq!(reader.peek(), None);
        assert_eq!(reader.next(), None);
    }
}
