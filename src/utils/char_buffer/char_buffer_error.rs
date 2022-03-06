use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CharBufferErrorType {
    
}

#[derive(Debug)]
pub enum CharBufferError {
    NotInBuffer(usize),
    NotEnoughSpace(char),
    Empty,
}

impl Display for CharBufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Only create messages when we want to print them
        match self {
            Self::NotEnoughSpace(c) => {
                write!(
                    f,
                    "Not enough space in the buffer to push the char {}",
                    c
                )
            },
            Self::Empty => write!(f, "The buffer is empty"),
            Self::NotInBuffer(pos) => write!(f, "The char at position {} is not in the buffer", pos),
        }
    }
}

impl Error for CharBufferError {}
