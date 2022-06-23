use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Debug)]
pub enum RingBufferError {
    NotEnoughSpace,
    OutOfBounds,
}

impl Display for RingBufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Only create messages when we want to print them
        match self {
            Self::NotEnoughSpace => write!(f, "Not enough space in the buffer to push data."),
            Self::OutOfBounds => write!(f, "Tried to read or write outside of the buffer."),
        }
    }
}

impl Error for RingBufferError {}
