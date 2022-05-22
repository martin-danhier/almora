use super::{Location, ParseInfo, ParserError, Span};

// Other
pub type ParseResult = Result<Option<ParseInfo>, ParserError>;

pub trait CreateParseResult {
    fn new(span: Span, len: usize) -> Self;
    fn matches(start: Location, end: Location) -> Self;
    fn no_match() -> Self;
    fn error(err: ParserError) -> Self;
    fn empty(start: Location) -> Self;
}

impl CreateParseResult for ParseResult {
    fn new(span: Span, len: usize) -> Self {
        Ok(Some(ParseInfo::new(span, len)))
    }

    fn matches(start: Location, end: Location) -> Self {
        Ok(Some(ParseInfo::new(
            Span::new(start, end),
            end.index() - start.index(),
        )))
    }

    fn no_match() -> Self {
        Ok(None)
    }

    fn error(err: ParserError) -> Self {
        Err(err)
    }

    fn empty(start: Location) -> Self {
        Ok(Some(ParseInfo::new(Span::new(start, start), 0)))
    }
}
