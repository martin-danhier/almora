use std::error::Error;

use crate::utils::ReadUTF8;

use super::{CharBufferError, CharBufferErrorType};

pub struct CharBuffer {
    buf: Vec<u8>,
    /// Where to read from next
    read_pos: usize,
    /// Where to write the next character.
    write_pos: usize,
    /// Number of chars in the buffer
    size: usize,
    /// Number of bytes in the buffer
    byte_size: usize,
}

impl CharBuffer {
    pub fn new(capacity: usize) -> Self {
        CharBuffer {
            buf: vec![0; capacity],
            read_pos: 0,
            write_pos: 0,
            size: 0,
            byte_size: 0,
        }
    }

    /// Pushes an unicode character to the buffer.
    ///
    /// An unicode char is a sequence of up to 4 bytes.
    ///
    /// Wraps around the buffer if necessary, but does not split the char in two parts.
    ///
    /// For example, if a 4 byte char is pushed, but there is only 2 slots
    /// left at the end of the buffer, the buffer will directly wrap around.
    /// This allows to return a slice from the buffer itself and avoid a memory allocation.
    pub fn push<'a>(&mut self, c: &'a str) -> Result<(), CharBufferError<'a>> {
        let c_len = c.len();

        // If there is not enough space at the write index, fill with 0 and wrap around
        if self.write_pos + c_len >= self.buf.capacity() {

            // If there is not enough space for the new char, return an error
            // Since there will be 0s from write_pos to the end, its as if write_pos is the new capacity
            if self.byte_size + c_len >= self.write_pos {
                return Err(CharBufferError::new(c, CharBufferErrorType::NotEnoughSpace));
            }

            // Fill the rest of the buffer with 0s
            for i in self.write_pos..self.buf.capacity() {
                self.buf[i] = 0;
            }
            self.byte_size += self.buf.capacity() - self.write_pos;
            self.write_pos = 0;
        }
        // If there is enough space, just copy the bytes
        else {
            // If there is not enough space for the new char, return an error
            if self.byte_size + c_len >= self.buf.capacity() {
                return Err(CharBufferError::new(c, CharBufferErrorType::NotEnoughSpace));
            }

            // Copy bytes
            for i in 0..c_len {
                self.buf[self.write_pos + i] = c.as_bytes()[i];
            }
            self.byte_size += c_len;
            self.write_pos += c_len;
        }

        Ok(())
    }

    pub fn consume(&mut self, n: usize) {}
}

impl<'a> ReadUTF8<'a> for CharBuffer {
    fn get_utf8(&'a self, start_index: usize) -> Option<(&'a str, usize)> {
        // Get the next utf8 character, wrapping around if necessary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() {
        let buffer = CharBuffer::new(10);

        assert!(buffer.buf.len() == 10);
        assert!(buffer.pos == 0);
        assert!(buffer.buf.iter().all(|&x| x == 0));
    }
}
