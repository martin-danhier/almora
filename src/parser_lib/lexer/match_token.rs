pub trait MatchToken {
    /// Tries to match the given string at the current position in the input.
    ///
    /// If it matches, the string is consumed and true is returned.
    fn match_str(&mut self, string: &'static str) -> bool;

    /// Tries to match an identifier at the current position in the input.
    ///
    /// If it matches, the string is consumed and the identifier is returned.
    ///
    /// An identifier is equivalent to [a-zA-Z_][a-zA-Z0-9_]*, where * is greedy.
    fn match_identifier(&mut self) -> Option<String>;
}
