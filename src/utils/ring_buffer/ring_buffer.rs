use crate::utils::{try_into_char::TryIntoChar, Peek};

use super::RingBufferError;
use std::fmt::{Debug, Display, Write};

/// Ring buffer for storing values.
#[derive(Debug)]
pub struct RingBuffer {
    // Use a buffer of bytes
    buf: Vec<u8>,
    /// Where to read from next
    read_pos: usize,
    /// Where to write the next character.
    write_pos: usize,
    /// Number of bytes in the buffer.
    len: usize,
}

impl RingBuffer {
    pub fn new(capacity: usize) -> Self {
        RingBuffer {
            buf: vec![0u8; capacity],
            read_pos: 0,
            write_pos: 0,
            len: 0,
        }
    }

    // Getters

    #[inline]
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn empty(&self) -> bool {
        self.len == 0
    }

    // Methods

    pub fn extend_bytes(&mut self, bytes: &[u8]) -> Result<(), RingBufferError> {
        if self.len() + bytes.len() > self.capacity() {
            return Err(RingBufferError::NotEnoughSpace);
        }

        let slice = &bytes[..];

        // Compute how much can be fit without wrapping
        let count_to_end = self.capacity() - self.write_pos;

        // If we can fit all the bytes, do it
        if count_to_end >= bytes.len() {
            self.buf[self.write_pos..self.write_pos + bytes.len()].copy_from_slice(slice);
            self.write_pos += bytes.len();
        }
        // Otherwise, copy the first part and the second part
        else {
            let capacity = self.capacity();
            self.buf[self.write_pos..capacity].copy_from_slice(&slice[..count_to_end]);
            self.buf[..bytes.len() - count_to_end].copy_from_slice(&slice[count_to_end..]);
            self.write_pos = bytes.len() - count_to_end;
        }

        // Increase size
        self.len += bytes.len();

        Ok(())
    }


    /// Compares if the given slice is equal to the beginning of the buffer.
    pub fn starts_with(&self, other: &[u8]) -> bool {
        // Compute how much can be fit without wrapping
        let mut count_to_end = self.capacity() - self.read_pos;

        // If we can fit all the bytes, do it
        if count_to_end >= other.len() {
            return &self.buf[self.read_pos..self.read_pos + other.len()] == other;
        }
        // Otherwise, copy the first part and the second part
        else {
            return &self.buf[self.read_pos..self.capacity()] == &other[..count_to_end]
                && &self.buf[..other.len() - count_to_end] == &other[count_to_end..];
        }
    }

    pub fn advance_by(&mut self, count: usize) {
        self.read_pos = (self.read_pos + count) % self.capacity();
        self.len -= count;
    }

    pub fn iter(&self) -> RingBufferIterator {
        RingBufferIterator {
            buffer: &self,
            index: self.read_pos,
        }
    }

}

pub struct RingBufferIterator<'a> {
    buffer: &'a RingBuffer,
    index: usize,
}

impl<'a> RingBufferIterator<'a> {
    pub fn advance(&mut self, count: usize) -> Result<(), RingBufferError> {
        // Simple case
        if count == 0 {
            return Ok(());
        }

        let index = (self.index + count) % self.buffer.capacity();

        // Case 1: _[R--I--[W__
        if self.buffer.read_pos <= index && index < self.buffer.write_pos {
            self.index = index;
            Ok(())
        }
        // Case 2: I-[W____[R-I
        else if !(self.buffer.write_pos <= index && index < self.buffer.read_pos) {
            self.index = index;
            Ok(())
        }
        else {
            Err(RingBufferError::OutOfBounds)
        }
    }
}

impl <'a> Iterator for RingBufferIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // Read the next UTF8 char
        let mut char_buf = [0u8; 4];
        let mut char_i = 0;

        // Read the bytes
        while char_i < 4 {
            // Reached the end
            if self.index == self.buffer.write_pos {
                return None;
            }

            char_buf[char_i] = self.buffer.buf[self.index];

            // Increment and wrap around if needed
            char_i += 1;
            self.index += 1;
            if self.index == self.buffer.capacity() {
                self.index = 0;
            }

            // Check that it is a valid char
            if let Ok(c) = char_buf.try_into_char() {
                // We have a full char
                return Some(c);
            }
        }

        // Not an UTF8 char, maybe likely, idk
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let cb = RingBuffer::new(10);

        assert_eq!(cb.capacity(), 10);
        assert_eq!(cb.read_pos, 0);
        assert_eq!(cb.write_pos, 0);

        for c in cb.buf {
            assert_eq!(c, 0u8);
        }
    }

    #[test]
    fn test_extend() {
        let mut cb = RingBuffer::new(15);

        let bytes = b"Hello \xF0\x9F\xA4\xA8";
        cb.extend_bytes(bytes).unwrap();

        assert_eq!(cb.len(), bytes.len());
        assert_eq!(cb.read_pos, 0);
        assert_eq!(cb.write_pos, bytes.len());
        assert!(cb.starts_with(b"Hello \xF0\x9F\xA4\xA8"));

        cb.advance_by(6);

        assert_eq!(cb.len(), bytes.len() - 6);
        assert_eq!(cb.read_pos, 6);
        assert_eq!(cb.write_pos, bytes.len());
        assert!(cb.starts_with(b"\xF0\x9F\xA4\xA8"));
    }

    #[test]
    fn test_iterator() {
        let mut cb = RingBuffer::new(15);

        let bytes = b"Hello ";
        cb.extend_bytes(bytes).unwrap();

        // Iterator allows to lookahead
        let mut iter = cb.iter();
        assert_eq!(iter.next(), Some('H'));
        assert_eq!(iter.next(), Some('e'));
        assert_eq!(iter.next(), Some('l'));
        assert_eq!(iter.next(), Some('l'));
        assert_eq!(iter.next(), Some('o'));
        assert_eq!(iter.next(), Some(' '));
        assert_eq!(iter.next(), None);

        // But we can still match the token, because it didn't consume the bytes
        assert!(cb.starts_with("Hello".as_bytes()));
    }
}
