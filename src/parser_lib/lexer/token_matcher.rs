use std::{fmt::Display, rc::Rc};

use crate::parser_lib::{CreateParseResult, Location, MatchStr, MatchToken, ParseResult, Stream};

/// In case of match, consumes the input to finish a token.
#[derive(Debug)]
pub struct TokenMatcher<R: MatchStr + Stream<char>> {
    value: Rc<dyn MatchToken<R>>,
}

impl<R: MatchStr + Stream<char>> TokenMatcher<R> {
    pub fn new(value: Rc<dyn MatchToken<R>>) -> Self {
        Self { value }
    }
}

impl<R: MatchStr + Stream<char>> MatchToken<R> for TokenMatcher<R> {
    fn test(&self, loc: &Location, reader: &mut R) -> ParseResult {
        if let Some(res) = self.value.test(loc, reader)? {
            // Consume the input
            reader.consume_nth(res.end().index() - 1);
            Ok(Some(res))
        } else {
            // If the value didn't match, the result is valid
            // Though, the span will be of length 0
            ParseResult::empty(*loc)
        }
    }
}

impl<R: MatchStr + Stream<char>> Display for TokenMatcher<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{ParseInfo, Span, StrMatcher, StringCharReader, Stream};

    use super::*;

    #[test]
    fn test_token_matcher() {
        let rule = TokenMatcher::new(Rc::new(StrMatcher::new("hello")));

        let mut reader = StringCharReader::new("hello world");

        // Should match
        let loc = Location::beginning();
        let info = ParseInfo::new(Span::new(loc, loc + 5), 5);
        let res = rule.test(&loc, &mut reader);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), Some(info));

        // Reader should now be at " world"
        assert_eq!(reader.peek(), Some(' '));
        
    }
}
