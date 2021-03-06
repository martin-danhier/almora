use std::fmt::Display;

use super::{Location, Span};

#[derive(Debug, PartialEq)]
/// Information about a successful parse
pub struct ParseInfo {
    span: Span,
    len: usize,
}

impl ParseInfo {
    pub fn new(span: Span, len: usize) -> Self {
        Self { span, len }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[allow(unused)]
    pub fn start(&self) -> &Location {
        self.span.start()
    }

    pub fn end(&self) -> &Location {
        self.span.end()
    }
}

impl Display for ParseInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.span)
    }
}
