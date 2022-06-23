use std::fmt::Debug;

use super::{ParserError, Stream};

pub trait MatchStr: Debug + Stream<char> {
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

    /// Checks if the next char is in the given char range.
    /// Avoids to check individually every possibility if the binary range is continuous.
    ///
    /// start: inclusive start of the range
    /// end: inclusive end of the range
    ///
    /// max: if 0, repeat until in doesn't match. If > 0, repeat max times.
    ///
    /// If Ok, returns the number of chars matched.
    fn match_range(
        &mut self,
        pos: usize,
        start: char,
        end: char,
        max: u8,
    ) -> Result<u32, ParserError>;

    /// Returns true if the char is a newline.
    fn is_newline(&mut self, pos: usize) -> Result<bool, ParserError>;

    /// Returns true if the char is the end of the input.
    fn is_end_of_input(&mut self, pos: usize) -> Result<bool, ParserError>;
}
