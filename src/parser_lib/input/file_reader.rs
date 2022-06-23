use std::{fs::File, io::Read};

use super::try_into_char::TryIntoChar;
use crate::{utils::{Peek, CharRingBuffer}, parser_lib::lexer::MatchToken};

// =========== Constants ===========

/// Capacity of the temporary buffer.
/// Large buffer means that large lookaheads are possible.
/// It also means that the file is able to read more chars at once,
/// and will need to do so less often.
const BUFFER_CAPACITY: usize = 1024;
/// Amount that the reader can read at at time. This allows the reader to bulk read chars,
/// and it will need to access the file less often. Cannot exceed BUFFER_CAPACITY.
const READ_SIZE: usize = BUFFER_CAPACITY / 2;

// =========== Structures ===========

pub struct FileReader {
    /// The file to read from
    file: File,
    /// Buffer to store looked ahead characters
    buffer: CharRingBuffer,
}

// =========== Implementations ===========

impl FileReader {
    pub fn new(file: File) -> Self {
        Self {
            file,
            buffer: CharRingBuffer::new(BUFFER_CAPACITY),
        }
    }

    /// Try to read `READ_SIZE` UTF-8 chars from the file.
    ///
    /// The resulting chars will be stored in the buffer. Therefore, there must be at least
    /// `READ_SIZE` free space in the buffer.
    ///
    /// If there is less that `READ_SIZE` characters in the file, this function will load everything until EOF.
    ///
    /// The function returns the actual number of read chars (<= `READ_SIZE`). A number strictly smaller than `READ_SIZE`
    /// means that EOF has been reached, and future calls to this function won't have any effect, and it will return 0.
    fn load_chars(&mut self) -> usize {
        // Check if there is enough space in the buffer, we don't want to override chars that weren't consumed.
        assert!(
            self.buffer.len() + READ_SIZE < BUFFER_CAPACITY,
            "Not enough space in buffer."
        );

        // Objective: read READ_SIZE chars. An UTF-8 char takes up to 4 bytes.

        // Buffer for the char we are reading
        let mut char_i = 0;
        let mut char_buf = [0u8; 4];

        // Buffer for read bytes
        let mut buf: Vec<u8> = Vec::with_capacity(READ_SIZE);
        let mut chars_to_read = READ_SIZE;
        let mut bytes_read = 1;

        while chars_to_read > 0 && bytes_read > 0 {
            // Try to read next bytes
            buf.resize(chars_to_read, 0);
            bytes_read = self.file.read(&mut buf).unwrap();

            // Try to find utf8 chars in the buffer
            for i in 0..bytes_read {
                char_buf[char_i] = buf[i];

                // Check that it is a valid char
                match char_buf.try_into_char() {
                    Ok(c) => {
                        self.buffer.push_back(c).expect("Buffer overflow");
                        // We can start the next char
                        char_i = 0;
                        char_buf = [0u8; 4];
                        chars_to_read -= 1;
                    }
                    // If it's not a valid char, we try by taking one more byte
                    Err(_) => {
                        char_i += 1;
                    }
                }
            }
        }

        // Return the number of chars read
        READ_SIZE - chars_to_read
    }
}

impl Iterator for FileReader {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // If there is nothing to read, try to read more chars
        if self.buffer.empty() {
            self.load_chars();
        }

        self.buffer.pop_front()
    }
}

impl Peek for FileReader {
    fn peek(&mut self) -> Option<Self::Item> {
        // If there is nothing to read, try to read more chars
        if self.buffer.empty() {
            self.load_chars();
        }

        self.buffer.peek()
    }

    fn peek_nth(&mut self, n: usize) -> Option<Self::Item> {
        if n > self.buffer.len() {
            self.load_chars();
        }

        self.buffer.peek_nth(n)
    }
}

impl MatchToken for FileReader {
    fn match_str(&self, string: &'static str) -> bool {
        if self.buffer.len() < string.len() {
            // We will need to expand it at one point. Do it now, that way the the buffer can match it in one go if possible
            self.load_chars();

            if self.buffer.len() < string.len() {
                // If still too small, then it can't match
                return false;
            }
        }







        false
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_reader() {
        let mut reader = FileReader::new(File::open("resources/test_files/test.txt").unwrap());

        assert_eq!(reader.peek(), Some('😎'));
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
        assert_eq!(reader.next(), Some('l'));

        assert_eq!(reader.peek(), Some('o'));
        assert_eq!(reader.next(), Some('o'));

        assert_eq!(reader.peek(), Some(' '));
        assert_eq!(reader.next(), Some(' '));

        assert_eq!(reader.peek(), Some('t'));
        assert_eq!(reader.next(), Some('t'));

        assert_eq!(reader.peek(), Some('h'));
        assert_eq!(reader.next(), Some('h'));

        assert_eq!(reader.peek(), Some('i'));
        assert_eq!(reader.next(), Some('i'));

        assert_eq!(reader.peek(), Some('s'));
        assert_eq!(reader.next(), Some('s'));

        assert_eq!(reader.peek(), Some(' '));
        assert_eq!(reader.next(), Some(' '));

        assert_eq!(reader.peek(), Some('i'));
        assert_eq!(reader.next(), Some('i'));

        assert_eq!(reader.peek(), Some('s'));
        assert_eq!(reader.next(), Some('s'));

        assert_eq!(reader.peek(), Some(' '));
        assert_eq!(reader.next(), Some(' '));

        assert_eq!(reader.peek(), Some('a'));
        assert_eq!(reader.next(), Some('a'));

        assert_eq!(reader.peek(), Some(' '));
        assert_eq!(reader.next(), Some(' '));

        assert_eq!(reader.peek(), Some('f'));
        assert_eq!(reader.next(), Some('f'));

        assert_eq!(reader.peek(), Some('i'));
        assert_eq!(reader.next(), Some('i'));

        assert_eq!(reader.peek(), Some('l'));
        assert_eq!(reader.next(), Some('l'));

        assert_eq!(reader.peek(), Some('e'));
        assert_eq!(reader.next(), Some('e'));
    }
}
