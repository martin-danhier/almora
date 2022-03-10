use std::fmt::Display;

use super::Location;

/// Location information of a range of characters in a source file.
///
/// Please note:
/// - start is **inclusive**
/// - end is **exclusive**
///
/// For example, the span of "hello" is
/// - start: (1, 1)
/// - end: (1, 6), which is the char just after "hello", where we would read next
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    start: Location,
    end: Location,
}

impl Span {
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> &Location {
        &self.start
    }

    pub fn end(&self) -> &Location {
        &self.end
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}