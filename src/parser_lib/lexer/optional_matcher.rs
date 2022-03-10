use std::fmt::Display;

use crate::parser_lib::{MatchToken, MatchStr, ParseResult, CreateParseResult, Location};

/// Matcher that returns true if the given matcher matches the string, or not
#[derive(Debug)]
pub struct OptionalMatcher<R: MatchStr> {
    value: Box<dyn MatchToken<R>>
}

impl<R: MatchStr> OptionalMatcher<R> {
    pub fn new(value: Box<dyn MatchToken<R>>) -> Self {
        Self {
            value
        }
    }
}

impl<R: MatchStr> MatchToken<R> for OptionalMatcher<R> {
    fn test(&self, loc: &Location, reader: &mut R) -> ParseResult {
        if let Ok(Some(res)) = self.value.test(loc, reader) {
            // If the value matched, the result is the same as the inner rule
            Ok(Some(res))
        }
        // Always matches
        else {
            // If the value didn't match, the result is still true because it is optional
            // Though, the span will be of length 0 since no actual characters were matched
            ParseResult::empty(*loc)
        }
    }
}

impl<R: MatchStr> Display for OptionalMatcher<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}?", self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{StringCharReader, StrMatcher, ParseInfo, Span};

    use super::*;

    #[test]
    fn test_optional_matcher() {
        let rule = OptionalMatcher::new(Box::new(StrMatcher::new("hello")));

        let mut reader = StringCharReader::new("hello world");

        // Test rule
        let loc = Location::beginning();
        let info = ParseInfo::new(Span::new(loc, Location::new(1, 6, 5)), 5);
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), Some(info));

        // If it does not match, should still match the empty string
        let loc2 = loc + 1;
        let info2 = ParseInfo::new(Span::new(loc2, loc2), 0);
        assert_eq!(rule.test(&loc2, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc2, &mut reader).unwrap(), Some(info2));

        let loc3 = loc + 6;
        let info3 = ParseInfo::new(Span::new(loc3, loc3), 0);
        assert_eq!(rule.test(&loc3, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc3, &mut reader).unwrap(), Some(info3));

    }
}