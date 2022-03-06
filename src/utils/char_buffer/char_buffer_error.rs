use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CharBufferErrorType {
    NotEnoughSpace,
}

#[derive(Debug)]
pub struct CharBufferError<'a> {
    pub c: &'a str,
    errorType: CharBufferErrorType,
}

impl<'a> CharBufferError<'a> {
    pub fn new(c: &'a str, errorType: CharBufferErrorType) -> Self {
        CharBufferError { c, errorType }
    }
}

impl<'a> Display for CharBufferError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Only create messages when we want to print them
        match self.errorType {
            CharBufferErrorType::NotEnoughSpace => {
                write!(
                    f,
                    "Not enough space in the buffer to push the char {}",
                    self.c
                )
            }
        }
    }
}

impl<'a> Error for CharBufferError<'a> {}
