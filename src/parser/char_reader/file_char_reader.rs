use std::{error::Error, fs::File, io::Read};

use crate::utils::CharBuffer;

use super::{utils::TryIntoChar, ReadChar};

/// Char reader that streams characters from a file.
/// Doesn't load the whole file into memory.
///
/// Maintains a buffer for peaked characters.
pub struct FileCharReader {
    /// The file to read from.
    file: File,
    /// The buffer of characters.
    buffer: CharBuffer,
    /// Number of UTF-8 characters read from the buffer (head).
    nb_read_from_buffer: usize,
    /// Number of UTF-8 characters read from the file (tail).
    nb_read_from_file: usize,
}

impl FileCharReader {
    /// Creates a new file char reader for the given file with the given buffer size
    pub fn new(filepath: &str, buffer_size: usize) -> Result<Self, Box<dyn Error>> {
        Ok(FileCharReader {
            file: File::open(filepath)?,
            buffer: CharBuffer::new(buffer_size),
            nb_read_from_file: 0,
            nb_read_from_buffer: 0,
        })
    }

    /// Try to load the next n utf8 chars into the buffer.
    /// Returns the number of actually loaded chars.
    /// 0 means either EOF, or not enough space in the buffer.
    pub fn load_chars(&mut self, n: usize) -> usize {
        // Check if there is enough space in the buffer, we don't want to override chars that weren't consumed
        if self.buffer.size() + n > self.buffer.capacity() {
            return 0;
        }

        // We want to load the next n bytes
        // An utf8 char takes up to 4 bytes

        // We can safely read n bytes at once, they count how many true utf8 chars there are
        // Then repeat with the number of remaining chars to read
        // This way, we can potentially avoid having to read each char individually

        // Buffer for the char we are reading
        let mut char_i = 0;
        let mut char_buf = [0u8; 4];

        // Buffer for read bytes
        let mut buf: Vec<u8> = Vec::with_capacity(n);

        // Stats
        let mut bytes_read = 1;
        let mut chars_to_read = n;

        while chars_to_read > 0 && bytes_read > 0 {
            // Create buffer
            buf.resize(chars_to_read, 0);

            // Try to read the next bytes
            bytes_read = self.file.read(&mut buf).unwrap();

            // Try to find utf8 chars in the buffer
            for i in 0..bytes_read {
                char_buf[char_i] = buf[i];

                // Check that it is a valid char
                match char_buf.try_into_char() {
                    Ok(c) => {
                        self.buffer.push(c).expect("Buffer overflow");
                        // We can start the next char
                        char_i = 0;
                        char_buf = [0u8; 4];
                        chars_to_read -= 1;
                        self.nb_read_from_file += 1;
                    }
                    // If it's not a valid char, we try by taking one more byte
                    Err(_) => {
                        char_i += 1;
                    }
                }
            }
        }

        // Return the number of chars read
        n - chars_to_read
    }

    /// Load chars in the buffer until the i is <= tail
    fn load_until(&mut self, index: usize) -> bool {
        if index >= self.nb_read_from_file {
            self.load_chars(index - self.nb_read_from_file + 1);

            if index >= self.nb_read_from_file  {
                return false;
            }
        }

        true
    }
}

impl ReadChar for FileCharReader {
    fn peek(&mut self) -> Option<char> {
        // Ensure that the next char is loaded
        self.load_until(self.nb_read_from_buffer);

        self.buffer.peek()
    }

    fn peek_nth(&mut self, n: usize) -> Option<char> {
        // Ensure that the nth char is loaded
        self.load_until(self.nb_read_from_buffer + n);

        self.buffer.peek_nth(n)
    }

    fn consume(&mut self) -> Option<char> {
        // Ensure that the next char is loaded
        self.load_until(self.nb_read_from_buffer);

        let res = self.buffer.pop();

        if res.is_some() {
            self.nb_read_from_buffer += 1;
        }

        res
    }

    fn consume_nth(&mut self, n: usize) -> Option<char> {
        // Ensure that the nth char is loaded
        self.load_until(self.nb_read_from_buffer + n);

        // Discard the chars before the nth
        for i in 0..n {
            self.buffer.pop();
        }

        let res = self.buffer.pop();
        if res.is_some() {
            self.nb_read_from_buffer += n + 1;
        }

        res
    }

    fn is_eof(&mut self) -> bool {
        // EOF = enable to load next char
        self.load_until(self.nb_read_from_buffer) == false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_chars() {
        let mut reader = FileCharReader::new("resources/test_files/test.txt", 10).unwrap();

        let res = reader.load_chars(10);
        assert_eq!(res, 10);

        // Check that the buffer was filled accordingly
        assert_eq!(reader.buffer.pop(), Some('😎'));
        assert_eq!(reader.buffer.pop(), Some(' '));
        assert_eq!(reader.buffer.pop(), Some('h'));
        assert_eq!(reader.buffer.pop(), Some('e'));
        assert_eq!(reader.buffer.pop(), Some('l'));
        assert_eq!(reader.buffer.pop(), Some('l'));
        assert_eq!(reader.buffer.pop(), Some('o'));
        assert_eq!(reader.buffer.pop(), Some(' '));
        assert_eq!(reader.buffer.pop(), Some('t'));
        assert_eq!(reader.buffer.pop(), Some('h'));
        assert_eq!(reader.buffer.pop(), None);
    }

    #[test]
    fn test_char_reader() {
        let mut reader = FileCharReader::new("resources/test_files/test.txt", 10).unwrap();

        assert_eq!(reader.peek(), Some('😎'));
        assert_eq!(reader.consume(), Some('😎'));

        assert_eq!(reader.peek(), Some(' '));
        assert_eq!(reader.consume(), Some(' '));

        assert_eq!(reader.peek(), Some('h'));
        assert_eq!(reader.consume(), Some('h'));

        assert_eq!(reader.peek(), Some('e'));
        assert_eq!(reader.consume(), Some('e'));

        assert_eq!(reader.peek(), Some('l'));
        assert_eq!(reader.consume(), Some('l'));

        assert_eq!(reader.peek(), Some('l'));
        assert_eq!(reader.consume(), Some('l'));

        assert_eq!(reader.peek(), Some('o'));
        assert_eq!(reader.consume(), Some('o'));    

        assert_eq!(reader.peek(), Some(' '));
        assert_eq!(reader.consume(), Some(' '));

        assert_eq!(reader.peek(), Some('t'));
        assert_eq!(reader.consume(), Some('t'));

        assert_eq!(reader.peek(), Some('h'));
        assert_eq!(reader.consume(), Some('h'));

        assert_eq!(reader.peek(), Some('i'));
        assert_eq!(reader.consume(), Some('i'));

        assert_eq!(reader.peek(), Some('s'));
        assert_eq!(reader.consume(), Some('s'));

        assert_eq!(reader.peek(), Some(' '));
        assert_eq!(reader.consume(), Some(' '));

        assert_eq!(reader.peek(), Some('i'));
        assert_eq!(reader.consume(), Some('i'));

        assert_eq!(reader.peek(), Some('s'));
        assert_eq!(reader.consume(), Some('s'));

        assert_eq!(reader.peek(), Some(' '));
        assert_eq!(reader.consume(), Some(' '));

        assert_eq!(reader.peek(), Some('a'));
        assert_eq!(reader.consume(), Some('a'));

        assert_eq!(reader.peek(), Some(' '));
        assert_eq!(reader.consume(), Some(' '));

        assert_eq!(reader.peek(), Some('f'));
        assert_eq!(reader.consume(), Some('f'));

        assert_eq!(reader.peek(), Some('i'));
        assert_eq!(reader.consume(), Some('i'));

        assert_eq!(reader.peek(), Some('l'));
        assert_eq!(reader.consume(), Some('l'));

        assert_eq!(reader.is_eof(), false);
        assert_eq!(reader.peek(), Some('e'));
        assert_eq!(reader.consume(), Some('e'));

        assert_eq!(reader.peek(), None);
        assert_eq!(reader.consume(), None);
        assert_eq!(reader.is_eof(), true);

    }

    #[test]
    fn test_consume_nth() {
        let mut reader = FileCharReader::new("resources/test_files/test.txt", 10).unwrap();

        assert_eq!(reader.peek_nth(9), Some('h'));
        assert_eq!(reader.consume_nth(9), Some('h'));
        assert_eq!(reader.consume(), Some('i'));
    }
}
