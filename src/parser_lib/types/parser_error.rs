use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParserError {
    /// Tried to peek a char which is before the cursor and thus not accessible anymore
    NoLookBehind(usize),
    /// Tried to peek a char which is too far away from the cursor and wouldn't fit in the buffer
    LookAheadBufferOverflow(usize),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ParserError::NoLookBehind(index)
                => write!(f, "Invalid search index: {}. Unable to look behind cursor.", index),
            ParserError::LookAheadBufferOverflow(index)
                => write!(f, "Could not look ahead char at relative index {}: char read buffer capacity is too small.", index),
        }
    }
}

impl Error for ParserError {}
