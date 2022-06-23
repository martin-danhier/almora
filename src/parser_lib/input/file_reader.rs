use std::{fs::File, io::Read};

use crate::{
    parser_lib::lexer::MatchToken,
    utils::{Peek, RingBuffer},
};

// =========== Constants ===========

/// Capacity of the temporary buffer.
/// Large buffer means that large lookaheads are possible.
/// It also means that the file is able to read more chars at once,
/// and will need to do so less often.
const BUFFER_CAPACITY: usize = 512;
/// Amount that the reader can read at at time. This allows the reader to bulk read chars,
/// and it will need to access the file less often. Cannot exceed BUFFER_CAPACITY.
const READ_SIZE: usize = BUFFER_CAPACITY / 2;

// =========== Structures ===========

pub struct FileReader {
    /// The file to read from
    file: File,
    /// Buffer to store looked ahead characters
    buffer: RingBuffer,
}

// =========== Implementations ===========

impl FileReader {
    pub fn new(file: File) -> Self {
        Self {
            file,
            buffer: RingBuffer::new(BUFFER_CAPACITY),
        }
    }

    /// Try to read `READ_SIZE` bytes from the file.
    ///
    /// The resulting chars will be stored in the buffer. Therefore, there must be at least
    /// `READ_SIZE` free space in the buffer.
    ///
    /// If there is less that `READ_SIZE` characters in the file, this function will load everything until EOF.
    ///
    /// The function returns the actual number of read bytes (<= `READ_SIZE`).
    fn load_bytes(&mut self) -> usize {
        // Check if there is enough space in the buffer, we don't want to override chars that weren't consumed.
        assert!(
            self.buffer.len() + READ_SIZE < BUFFER_CAPACITY,
            "Not enough space in buffer."
        );

        let mut buf = vec![0u8; READ_SIZE];
        let bytes_read = self.file.read(&mut buf).unwrap();

        // Store the read chars in the buffer
        self.buffer
            .extend_bytes(&buf[..bytes_read])
            .expect("Could not extend buffer.");

        // Return the number of bytes read
        bytes_read
    }
}

impl MatchToken for FileReader {
    fn match_str(&mut self, string: &'static str) -> bool {
        if self.buffer.len() < string.len() {
            // We will need to expand it at one point. Do it now, that way the the buffer can match it in one go if possible
            self.load_bytes();

            if self.buffer.len() < string.len() {
                // If still too small, then it can't match
                return false;
            }
        }

        if self.buffer.starts_with(string.as_bytes()) {
            // Consume the chars
            self.buffer.advance_by(string.len());
            true
        } else {
            false
        }
    }

    fn match_identifier(&mut self) -> Option<String> {
        let mut ident = String::new();

        'outer: loop {
            // The buffer only contains the identifier and we reached the end.
            // Maybe it goes even further, so we need to load more chars.
            if self.buffer.len() == ident.len() {
                self.load_bytes();

                // Nothing changed: we can stop
                if self.buffer.len() == ident.len() {
                    break;
                }
            }

            let mut iter = self.buffer.iter();
            // Go back to where we left off
            iter.advance(ident.len()).expect("Buffer modified.");
            while let Some(c) = iter.next() {
                if c.is_ascii_alphabetic() || c == '_' || (!ident.is_empty() && c.is_digit(10)) {
                    ident.push(c);
                } else {
                    // It stopped matching, we can stop
                    break 'outer;
                }
            }
        }

        match ident.is_empty() {
            true => None,
            false => {
                self.buffer.advance_by(ident.len());
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
        let mut reader = FileReader::new(File::open("resources/test_files/test.txt").unwrap());

        assert!(reader.match_str("😎 "));
        assert!(reader.match_str("hello "));
        assert!(!reader.match_str("world"));
        assert_eq!(reader.match_identifier(), Some("this".to_string()));
        assert!(reader.match_str(" "));
        assert_eq!(reader.match_identifier(), Some("is".to_string()));
        assert!(reader.match_str(" "));
        assert!(reader.match_str("a file which is really important and useful"));
    }
}
