use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Debug)]
pub enum RingBufferError {
    NotEnoughSpace(char),
}

impl Display for RingBufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Only create messages when we want to print them
        match self {
            Self::NotEnoughSpace(c) => {
                write!(f, "Not enough space in the buffer to push the char {}", c)
            }
        }
    }
}

impl Error for RingBufferError {}
