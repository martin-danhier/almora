use std::fmt::Debug;

use super::ParserError;

pub trait MatchStr: Debug {
    /// Compares the given string `s` with the input at the position `pos`.
    ///
    /// The position is an absolute index from the start of the input.
    ///
    /// Returns `false` if s is a substring of the input starting at pos `pos`, `false` otherwise.
    ///
    /// Can return an error if:
    /// - The given pos is behind the cursor (no look behind)
    /// - The given pos + the size of the string falls outside of the size of the buffer (look ahead overflow)
    fn match_str(&mut self, pos: usize, s: &str) -> Result<bool, ParserError>;
}