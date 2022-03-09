use std::{error::Error, fmt::{Display, Debug}};

#[derive(Debug)]
pub enum RingBufferError<T: Copy + Clone + Debug + Display> {
    NotInBuffer(usize),
    NotEnoughSpace(T),
}

impl<T: Copy + Clone + Debug + Display> Display for RingBufferError<T> {
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
            Self::NotInBuffer(pos) => write!(f, "The char at position {} is not in the buffer", pos),
        }
    }
}

impl<T: Copy + Clone + Debug + Display> Error for RingBufferError<T> {}
