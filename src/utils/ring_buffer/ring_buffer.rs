use crate::utils::Peek;

use super::RingBufferError;
use std::fmt::{Debug, Display, Write};

/// Ring buffer for storing values.
#[derive(Debug)]
pub struct CharRingBuffer {
    // Use a String for the buffer
    // That will allow to take slices for quick comparisons
    buf: String,
    /// Where to read from next
    read_pos: usize,
    /// Where to write the next character.
    write_pos: usize,
    /// Number of chars in the buffer.
    len: usize,
}

impl CharRingBuffer {
    pub fn new(capacity: usize) -> Self {
        CharRingBuffer {
            buf: String::with_capacity(capacity),
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

    /// Append a value at the end of the buffer
    pub fn push_back(&mut self, c: char) -> Result<(), RingBufferError> {
        if self.len() == self.capacity() {
            return Err(RingBufferError::NotEnoughSpace(c));
        }

        self.buf[self.write_pos] = c;


        self.buf.write_char(c)
        // Increase write_pos and size and wrap around if necessary
        self.write_pos += 1;
        if self.write_pos == self.capacity() {
            self.write_pos = 0;
        }
        // Increase size
        self.len += 1;

        Ok(())
    }

    /// Remove the value at the start of the buffer and return it.
    pub fn pop_front(&mut self) -> Option<char> {
        if self.empty() {
            return None;
        }

        let c = self.buf.chars().nth(self.read_pos);

        // Increase read_pos and size and wrap around if necessary
        self.read_pos += 1;
        if self.read_pos == self.capacity() {
            self.read_pos = 0;
        }
        // Decrease size
        self.len -= 1;

        c
    }

    fn peek(&mut self) -> Option<char> {
        if self.empty() {
            return None;
        }

        self.buf.chars().nth(self.read_pos)
    }

    fn peek_nth(&mut self, n: usize) -> Option<char> {
        if self.empty(){
            return None;
        }

        if n >= self.len() {
            return None;
        }

        let pos = (self.read_pos + n) % self.capacity();
        self.buf.chars().nth(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let cb = CharRingBuffer::<char>::new(10);

        assert_eq!(cb.capacity(), 10);
        assert_eq!(cb.read_pos, 0);
        assert_eq!(cb.write_pos, 0);

        for c in cb.buf {
            assert_eq!(c, None);
        }
    }

    #[test]
    fn test_push() {
        let mut cb = CharRingBuffer::new(5);

        // Move head to middle so we can test wrapping
        cb.write_pos = 2;

        assert_eq!(cb.len(), 0);

        assert_eq!(cb.push_back('h').is_ok(), true);
        assert_eq!(cb.len(), 1);
        assert_eq!(cb.write_pos, 3);
        assert_eq!(cb.buf[2], Some('h'));

        assert_eq!(cb.push_back('e').is_ok(), true);
        assert_eq!(cb.len(), 2);
        assert_eq!(cb.write_pos, 4);
        assert_eq!(cb.buf[3], Some('e'));

        assert_eq!(cb.push_back('l').is_ok(), true);
        assert_eq!(cb.len(), 3);
        assert_eq!(cb.write_pos, 0);
        assert_eq!(cb.buf[4], Some('l'));

        assert_eq!(cb.push_back('l').is_ok(), true);
        assert_eq!(cb.len(), 4);
        assert_eq!(cb.write_pos, 1);
        assert_eq!(cb.buf[0], Some('l'));

        assert_eq!(cb.push_back('o').is_ok(), true);
        assert_eq!(cb.len(), 5);
        assert_eq!(cb.write_pos, 2);
        assert_eq!(cb.buf[1], Some('o'));

        // Now we should be full
        assert_eq!(cb.push_back('!').is_ok(), false);
    }

    #[test]
    fn test_pop() {
        let mut cb = CharRingBuffer::new(5);

        // Move head to middle so we can test wrapping
        cb.read_pos = 2;
        cb.write_pos = 2;

        // First, its empty
        assert_eq!(cb.len(), 0);
        assert_eq!(cb.pop_front().is_none(), true);

        // Now we push some chars
        assert_eq!(cb.push_back('h').is_ok(), true);
        assert_eq!(cb.push_back('e').is_ok(), true);

        // Now we should have 2 chars
        assert_eq!(cb.len(), 2);

        // Pop the first char
        assert_eq!(cb.pop_front().unwrap(), 'h');
        assert_eq!(cb.len(), 1);
        assert_eq!(cb.read_pos, 3);

        // Pop the second char
        assert_eq!(cb.pop_front().unwrap(), 'e');
        assert_eq!(cb.len(), 0);
        assert_eq!(cb.read_pos, 4);

        // Now we should be empty
        assert_eq!(cb.pop_front().is_none(), true);

        // Now we push some more chars
        assert_eq!(cb.push_back('h').is_ok(), true);
        assert_eq!(cb.push_back('e').is_ok(), true);
        assert_eq!(cb.push_back('l').is_ok(), true);
        assert_eq!(cb.push_back('l').is_ok(), true);
        assert_eq!(cb.push_back('o').is_ok(), true);

        // Now we should have 5 chars
        assert_eq!(cb.len(), 5);

        // Pop the first char
        assert_eq!(cb.pop_front().unwrap(), 'h');
        assert_eq!(cb.len(), 4);
        assert_eq!(cb.read_pos, 0);

        // Pop the second char
        assert_eq!(cb.pop_front().unwrap(), 'e');
        assert_eq!(cb.len(), 3);
        assert_eq!(cb.read_pos, 1);
    }

    #[test]
    fn test_peek() {
        let mut cb = CharRingBuffer::new(5);

        // Move head to middle so we can test wrapping
        cb.read_pos = 2;
        cb.write_pos = 2;

        // First, its empty
        assert_eq!(cb.len(), 0);
        assert_eq!(cb.peek().is_none(), true);

        // Now we push some chars
        assert_eq!(cb.push_back('h').is_ok(), true);
        assert_eq!(cb.push_back('e').is_ok(), true);

        // Now we should have 2 chars
        assert_eq!(cb.len(), 2);

        // Peek the first char
        assert_eq!(cb.peek().unwrap(), 'h');
        assert_eq!(cb.len(), 2);
        assert_eq!(cb.read_pos, 2);

        // Peek with nth
        assert_eq!(cb.peek_nth(0).unwrap(), 'h');
        assert_eq!(cb.len(), 2);
        assert_eq!(cb.read_pos, 2);

        assert_eq!(cb.peek_nth(1).unwrap(), 'e');
        assert_eq!(cb.len(), 2);
        assert_eq!(cb.read_pos, 2);

        assert_eq!(cb.peek_nth(2).is_none(), true);
    }
}
