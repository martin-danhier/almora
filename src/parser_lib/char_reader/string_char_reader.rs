use crate::parser_lib::{Stream, MatchStr, ParseResult, ParserError};

/// Char reader that streams characters from a string.
///
/// Since the whole string is loaded in memory, doesn't use a buffer.
/// Since Rust peekable iterator do not support look ahead of more than 1 char, this doesn't use it.
/// Thus, it is not at all as optimized as the file reader .
///
/// Useful for testing. In real life situations, prefer a FileCharReader.
#[derive(Debug)]
pub struct StringCharReader {
    string: String,
    /// The current position in the string.
    cursor_index: usize,
}

impl StringCharReader {
    /// Creates a new StringCharReader from a string.
    pub fn new(s: &str) -> Self {
        Self {
            string: String::from(s),
            cursor_index: 0,
        }
    }
}

impl Stream<char> for StringCharReader {
    fn peek(&mut self) -> Option<char> {
        self.string.chars().nth(self.cursor_index)
    }

    fn peek_nth(&mut self, n: usize) -> Option<char> {
        self.string.chars().nth(self.cursor_index + n)
    }

    fn consume(&mut self) -> Option<char> {
        let c = self.peek()?;

        // If there is a char, return it
        self.cursor_index += 1;
        Some(c)
    }

    fn consume_nth(&mut self, n: usize) -> Option<char> {
        let c = self.peek_nth(n)?;

        // If there is a char, return it
        self.cursor_index += n + 1;
        Some(c)
    }

    fn is_eof(&mut self) -> bool {
        self.string.chars().nth(self.cursor_index) == None
    }
}

impl MatchStr for StringCharReader {
    fn match_str(&mut self, pos: usize, s: &str) -> Result<bool, ParserError> {
        if pos < self.cursor_index {
            return Err(ParserError::NoLookBehind(pos));
        }

        // This is the amount by which we will need to look ahead for the start of the stream
        let relative_pos = pos - self.cursor_index;

        // Compare each char
        let mut i = relative_pos;
        for str_c in s.chars() {
            if let Some(file_c) = self.peek_nth(i) {
                if file_c != str_c {
                    // If a difference is found, it's not equal
                    return Ok(false);
                }
            }
            else {
                // If EOF is reached before the end of the string to compare, it's not equal
                return Ok(false);
            }
            i += 1;
        }

        Ok(true)
    }

    fn match_range(&mut self, pos: usize, start: char, end: char, max: u8) -> Result<u32, ParserError> {
        if pos < self.cursor_index {
            return Err(ParserError::NoLookBehind(pos));
        }

        // This is the amount by which we will need to look ahead for the start of the stream
        let relative_pos = pos - self.cursor_index;

        let mut matched = 0;

        // Compare each char
        let mut i = relative_pos;
        while let Some(c) = self.peek_nth(i) {
            // If a difference is found, or if we already have matched the max, we stop here
            if c < start || c > end {
                break;
            }

            // If there is a max and it is reached, we stop here
            if max != 0 && matched >= max.into() {
                break;
            }

            matched += 1;
            i += 1;
        }

        Ok(matched)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_char_reader() {
        let mut reader = StringCharReader::new("hello");

        // Not EOF
        assert_eq!(reader.is_eof(), false);

        // Try peeking
        assert_eq!(reader.peek(), Some('h'));
        assert_eq!(reader.peek_nth(0), Some('h'));
        assert_eq!(reader.peek_nth(1), Some('e'));
        assert_eq!(reader.peek_nth(2), Some('l'));
        assert_eq!(reader.peek_nth(3), Some('l'));
        assert_eq!(reader.peek_nth(4), Some('o'));
        assert_eq!(reader.peek_nth(5), None);

        // Try consuming some chars
        assert_eq!(reader.consume(), Some('h'));
        assert_eq!(reader.consume(), Some('e'));

        // Still not EOF
        assert_eq!(reader.is_eof(), false);

        // Try peeking again
        assert_eq!(reader.peek(), Some('l'));
        assert_eq!(reader.peek_nth(0), Some('l'));
        assert_eq!(reader.peek_nth(1), Some('l'));
        assert_eq!(reader.peek_nth(2), Some('o'));
        assert_eq!(reader.peek_nth(3), None);

        // Try consuming some more chars
        assert_eq!(reader.consume_nth(2), Some('o'));

        // Try peeking again
        assert_eq!(reader.peek(), None);
        assert_eq!(reader.peek_nth(0), None);

        // Try consuming again
        assert_eq!(reader.consume(), None);
        assert_eq!(reader.consume_nth(0), None);

        // Indeed, we should have EOF
        assert_eq!(reader.is_eof(), true);
    }

    #[test]
    fn test_utf8() {
        let mut reader = StringCharReader::new("👀🍕");

        // Not EOF
        assert_eq!(reader.is_eof(), false);

        // Try peeking
        assert_eq!(reader.peek(), Some('👀'));
        assert_eq!(reader.peek_nth(0), Some('👀'));
        assert_eq!(reader.peek_nth(1), Some('🍕'));

        // Try consuming some chars
        assert_eq!(reader.consume(), Some('👀'));
        assert_eq!(reader.consume(), Some('🍕'));

        // EOF
        assert_eq!(reader.is_eof(), true);

        // Try peeking again
        assert_eq!(reader.peek(), None);
        assert_eq!(reader.peek_nth(0), None);

        // Try consuming again
        assert_eq!(reader.consume(), None);
        assert_eq!(reader.consume_nth(0), None);

    }

    #[test]
    fn test_match_str() {
        let mut reader = StringCharReader::new("😎 hello this is a file which is really important and useful");

        // Look ahead check should work
        assert!(reader.match_str(8, "this").is_ok());
        assert_eq!(reader.match_str(8, "this").unwrap(), true);

        // But shifted by some chars it doesn't work anymore
        assert!(reader.match_str(10, "this").is_ok());
        assert_eq!(reader.match_str(10, "this").unwrap(), false);

        // Since the buffer is big it even works when the word is far away
        assert!(reader.match_str(39, "important").is_ok());
        assert_eq!(reader.match_str(39, "important").unwrap(), true);

        // We can still compare words at the beginning, since the cursor hasn't moved
        assert!(reader.match_str(2, "hello").is_ok());
        assert_eq!(reader.match_str(2, "hello").unwrap(), true);

        // Now, let's try to consume some chars at the beginning
        assert_eq!(reader.consume_nth(6), Some('o'));

        // We shouldn't be able to access the "hello" word anymore
        assert!(reader.match_str(2, "hello").is_err());
        assert_eq!(reader.match_str(2, "hello").unwrap_err(), ParserError::NoLookBehind(2));
    }

    #[test]
    fn test_range() {
        let mut reader = StringCharReader::new("😎 hello this is a file which is really important and useful");

        // Look ahead check should work
        assert!(reader.match_range(9, 'a', 'z', 1).is_ok());
        assert_eq!(reader.match_range(9, 'a', 'z', 1).unwrap(), 1);

        // But not capital
        assert!(reader.match_range(9, 'A', 'Z', 1).is_ok());
        assert_eq!(reader.match_range(9, 'A', 'Z', 1).unwrap(), 0);

        // But not numbers
        assert!(reader.match_range(9, '0', '9', 1).is_ok());
        assert_eq!(reader.match_range(9, '0', '9', 1).unwrap(), 0);

        // Space is no alpha numeric
        assert!(reader.match_range(7, 'a', 'z', 1).is_ok());
        assert_eq!(reader.match_range(7, 'a', 'z', 1).unwrap(), 0);

        assert!(reader.match_range(7, 'A', 'Z', 1).is_ok());
        assert_eq!(reader.match_range(7, 'A', 'Z', 1).unwrap(), 0);

        assert!(reader.match_range(7, '0', '9', 1).is_ok());
        assert_eq!(reader.match_range(7, '0', '9', 1).unwrap(), 0);

        // Should also work for longer matches
        // Here it can get words up to 10 chars, but it stops at the space so it only finds 4 chars
        assert!(reader.match_range(8, 'a', 'z', 10).is_ok());
        assert_eq!(reader.match_range(8, 'a', 'z', 10).unwrap(), 4);

        // 0 is infinite max
        assert!(reader.match_range(39, 'a', 'z', 0).is_ok());
        assert_eq!(reader.match_range(39, 'a', 'z', 0).unwrap(), 9);
    }
}