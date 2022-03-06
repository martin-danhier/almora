use super::{CharBufferError};

/// Ring buffer for storing chars.
pub struct CharBuffer {
    buf: Vec<char>,
    /// Where to read from next
    read_pos: usize,
    /// Where to write the next character.
    write_pos: usize,
    /// Number of chars in the buffer.
    size: usize,
}

impl CharBuffer {
    pub fn new(capacity: usize) -> Self {
        CharBuffer {
            buf: vec!['\0'; capacity],
            read_pos: 0,
            write_pos: 0,
            size: 0,
        }
    }

    // Getters

    #[inline]
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    // Methods
    pub fn push(&mut self, c: char) -> Result<(), CharBufferError> {
        if self.size() == self.capacity() {
            return Err(CharBufferError::NotEnoughSpace(c));
        }

        self.buf[self.write_pos] = c;
        // Increase write_pos and size and wrap around if necessary
        self.write_pos += 1;
        if self.write_pos == self.capacity() {
            self.write_pos = 0;
        }
        // Increase size
        self.size += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.size() == 0 {
            return None;
        }

        let c = self.buf[self.read_pos];
        // Increase read_pos and size and wrap around if necessary
        self.read_pos += 1;
        if self.read_pos == self.capacity() {
            self.read_pos = 0;
        }
        // Decrease size
        self.size -= 1;

        Some(c)
    }

    pub fn peek(&self) -> Option<char> {
        if self.size() == 0 {
            return None;
        }

        Some(self.buf[self.read_pos])
    }

    pub fn peek_nth(&self, n: usize) -> Option<char> {
        if self.size() == 0 {
            return None;
        }

        if n >= self.size() {
            return None;
        }

        let pos = (self.read_pos + n) % self.capacity();
        Some(self.buf[pos])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let cb = CharBuffer::new(10);

        assert_eq!(cb.capacity(), 10);
        assert_eq!(cb.read_pos, 0);
        assert_eq!(cb.write_pos, 0);

        for c in cb.buf {
            assert_eq!(c, '\0');
        }
    }

    #[test]
    fn test_push() {
        let mut cb = CharBuffer::new(5);

        // Move head to middle so we can test wrapping
        cb.write_pos = 2;

        assert_eq!(cb.size(), 0);

        assert_eq!(cb.push('h').is_ok(), true);
        assert_eq!(cb.size(), 1);
        assert_eq!(cb.write_pos, 3);
        assert_eq!(cb.buf[2], 'h');

        assert_eq!(cb.push('e').is_ok(), true);
        assert_eq!(cb.size(), 2);
        assert_eq!(cb.write_pos, 4);
        assert_eq!(cb.buf[3], 'e');

        assert_eq!(cb.push('l').is_ok(), true);
        assert_eq!(cb.size(), 3);
        assert_eq!(cb.write_pos, 0);
        assert_eq!(cb.buf[4], 'l');

        assert_eq!(cb.push('l').is_ok(), true);
        assert_eq!(cb.size(), 4);
        assert_eq!(cb.write_pos, 1);
        assert_eq!(cb.buf[0], 'l');

        assert_eq!(cb.push('o').is_ok(), true);
        assert_eq!(cb.size(), 5);
        assert_eq!(cb.write_pos, 2);
        assert_eq!(cb.buf[1], 'o');

        // Now we should be full
        assert_eq!(cb.push('!').is_ok(), false);
    }

    #[test]
    fn test_pop() {
        let mut cb = CharBuffer::new(5);

        // Move head to middle so we can test wrapping
        cb.read_pos = 2;
        cb.write_pos = 2;

        // First, its empty
        assert_eq!(cb.size(), 0);
        assert_eq!(cb.pop().is_none(), true);

        // Now we push some chars
        assert_eq!(cb.push('h').is_ok(), true);
        assert_eq!(cb.push('e').is_ok(), true);

        // Now we should have 2 chars
        assert_eq!(cb.size(), 2);

        // Pop the first char
        assert_eq!(cb.pop().unwrap(), 'h');
        assert_eq!(cb.size(), 1);
        assert_eq!(cb.read_pos, 3);

        // Pop the second char
        assert_eq!(cb.pop().unwrap(), 'e');
        assert_eq!(cb.size(), 0);
        assert_eq!(cb.read_pos, 4);

        // Now we should be empty
        assert_eq!(cb.pop().is_none(), true);

        // Now we push some more chars
        assert_eq!(cb.push('h').is_ok(), true);
        assert_eq!(cb.push('e').is_ok(), true);
        assert_eq!(cb.push('l').is_ok(), true);
        assert_eq!(cb.push('l').is_ok(), true);
        assert_eq!(cb.push('o').is_ok(), true);

        // Now we should have 5 chars
        assert_eq!(cb.size(), 5);

        // Pop the first char
        assert_eq!(cb.pop().unwrap(), 'h');
        assert_eq!(cb.size(), 4);
        assert_eq!(cb.read_pos, 0);

        // Pop the second char
        assert_eq!(cb.pop().unwrap(), 'e');
        assert_eq!(cb.size(), 3);
        assert_eq!(cb.read_pos, 1);
    }

    #[test]
    fn test_peek() {
        let mut cb = CharBuffer::new(5);

        // Move head to middle so we can test wrapping
        cb.read_pos = 2;
        cb.write_pos = 2;

        // First, its empty
        assert_eq!(cb.size(), 0);
        assert_eq!(cb.peek().is_none(), true);

        // Now we push some chars
        assert_eq!(cb.push('h').is_ok(), true);
        assert_eq!(cb.push('e').is_ok(), true);

        // Now we should have 2 chars
        assert_eq!(cb.size(), 2);

        // Peek the first char
        assert_eq!(cb.peek().unwrap(), 'h');
        assert_eq!(cb.size(), 2);
        assert_eq!(cb.read_pos, 2);

        // Peek with nth
        assert_eq!(cb.peek_nth(0).unwrap(), 'h');
        assert_eq!(cb.size(), 2);
        assert_eq!(cb.read_pos, 2);

        assert_eq!(cb.peek_nth(1).unwrap(), 'e');
        assert_eq!(cb.size(), 2);
        assert_eq!(cb.read_pos, 2);

        assert_eq!(cb.peek_nth(2).is_none(), true);
    }

}
