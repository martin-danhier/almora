use std::{fmt::Display, rc::Rc};

use crate::parser_lib::{CreateParseResult, Location, MatchStr, MatchToken, ParseResult};

/// Matcher that returns true if the given matcher doesn't match the string
#[derive(Debug)]
pub struct NotMatcher<R: MatchStr> {
    value: Rc<dyn MatchToken<R>>,
}

impl<R: MatchStr> NotMatcher<R> {
    pub fn new(value: Rc<dyn MatchToken<R>>) -> Self {
        Self { value }
    }
}

impl<R: MatchStr> MatchToken<R> for NotMatcher<R> {
    fn test(&self, loc: &Location, reader: &mut R) -> ParseResult {
        if let Some(_) = self.value.test(loc, reader)? {
            // If the value matched, this is not a match
            ParseResult::no_match()
        } else {
            // If the value didn't match, the result is valid
            // Though, the span will be of length 0
            ParseResult::empty(*loc)
        }
    }
}

impl<R: MatchStr> Display for NotMatcher<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(!{})", self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{ParseInfo, Span, StrMatcher, StringCharReader};

    use super::*;

    #[test]
    fn test_not_matcher() {
        let rule = NotMatcher::new(Rc::new(StrMatcher::new("hello")));

        let mut reader = StringCharReader::new("hello world");

        // Test rule
        let loc = Location::beginning();
        // Shouldn't match
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), None);

        // Should match empty string if there is no match
        let loc2 = loc + 1;
        let info2 = ParseInfo::new(Span::new(loc2, loc2), 0);
        assert_eq!(rule.test(&loc2, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc2, &mut reader).unwrap(), Some(info2));

        // String representation should be "(!\"hello\"")"
        assert_eq!(rule.to_string(), "(!\"hello\")");
    }
}
