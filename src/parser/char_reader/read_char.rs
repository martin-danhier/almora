pub trait ReadChar {
    /// Returns the next char in the input
    fn peek(&mut self) -> Option<char>;

    /// Returns the nth next char in the input
    /// 
    /// For example, if the next chars from the cursor are "hello",
    /// then `peek_nth(4)` returns `Some('o')`
    fn peek_nth(&mut self, n: usize) -> Option<char>;

    /// Consumes the next char in the input and returns it
    fn consume(&mut self) -> Option<char>;

    /// Returns the nth next char in the input and consumes everything up to it
    /// 
    /// For example, if the next chars from the cursor are "hello world",
    /// then `consume_nth(4)` returns `Some('o')` and the cursor is now at `" world"`
    /// 
    /// All the chars before it are thus lost if they were not peeked before
    fn consume_nth(&mut self, n: usize) -> Option<char>;

    /// Checks whether the end of the input has been reached
    fn is_eof(&mut self) -> bool;
}