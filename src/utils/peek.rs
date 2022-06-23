/// Extension of iterator that allows to peek elements
pub trait Peek: Iterator {
    /// Returns the next value without consuming it.
    ///
    /// Calling this multiple times in a row doesn't change the outcome.
    ///
    /// Returns None if the end of the iterator is reached.
    fn peek(&mut self) -> Option<Self::Item>;

    /// Returns the nth value (starting from the current position) without consuming it.
    ///
    /// Calling this multiple times in a row doesn't change the outcome.
    ///
    /// Returns None if the end of the iterator is reached, or the requested position is too far away.
    fn peek_nth(&mut self, n: usize) -> Option<Self::Item>;
}