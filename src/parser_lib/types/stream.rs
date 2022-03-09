pub trait Stream<T> {
    /// Returns the next elem in the input
    fn peek(&mut self) -> Option<T>;

    /// Returns the nth next elem in the input
    /// 
    /// For example, if the next elements from the cursor are "hello",
    /// then `peek_nth(4)` returns `Some('o')`
    fn peek_nth(&mut self, n: usize) -> Option<T>;

    /// Consumes the next elem in the input and returns it
    fn consume(&mut self) -> Option<T>;

    /// Returns the nth next elem in the input and consumes everything up to it
    /// 
    /// For example, if the next elems from the cursor are "hello world",
    /// then `consume_nth(4)` returns `Some('o')` and the cursor is now at `" world"`
    /// 
    /// All the elems before it are thus lost if they were not peeked before
    fn consume_nth(&mut self, n: usize) -> Option<T>;

    /// Checks whether the end of the input has been reached
    fn is_eof(&mut self) -> bool;
}