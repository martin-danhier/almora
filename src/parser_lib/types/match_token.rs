use std::fmt::{Display, Debug};

use super::{MatchStr, ParseResult, Location};

/// A matcher (or parser) tells how to analyse a specific part of the source code.
///
/// For example, a "StringMatcher" will try to match an exact string.
pub trait MatchToken<R: MatchStr>: Display + Debug {
    /// Compares this token to the input at the given location in the reader.
    ///
    /// Returns true if the token matches, false otherwise.
    ///
    /// Propagates errors returned by the reader.
    fn test(&self, loc: &Location, reader: &mut R) -> ParseResult;
}
