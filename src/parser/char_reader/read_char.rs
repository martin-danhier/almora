pub trait ReadChar<'a> {
    /// Returns the next char in the input
    fn peak() -> &'a str;

    /// Returns the next n chars in the input
    fn peak_n(n: i32) -> &'a str;

    /// Consumes the next char in the input and returns it
    fn consume() -> &'a str;

    /// Consumes the next n chars in the input and returns them
    fn consume_n(n: i32) -> &'a str;

    /// Checks whether the end of the input has been reached
    fn is_eof() -> bool;
}