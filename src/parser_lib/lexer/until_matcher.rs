use std::{fmt::Display, rc::Rc};

use crate::parser_lib::{CreateParseResult, Location, MatchStr, MatchToken, ParseResult};

/// Matcher that tries to match as many characters as possible until the given matcher matches
#[derive(Debug)]
pub struct UntilMatcher<R: MatchStr> {
    until: Rc<dyn MatchToken<R>>,
    min: usize,
}

impl<R: MatchStr> UntilMatcher<R> {
    pub fn new(until: Rc<dyn MatchToken<R>>, min: usize) -> Self {
        Self { until, min }
    }
}

impl<R: MatchStr> MatchToken<R> for UntilMatcher<R> {
    fn test(&self, loc: &Location, reader: &mut R) -> ParseResult {
        let mut count = 0;
        let mut end_loc = *loc;

        // Try to match the matcher at the end until it works
        while let Ok(None) = self.until.test(&end_loc, reader) {
            // If the EOF is reached, stop the match there
            if reader.is_end_of_input(end_loc.index())? {
                break;
            }

            // We got one more match
            count += 1;

            // The end location is thus further
            // We have to check if we are at a new line or not to increment the location
            if reader.is_newline(end_loc.index())? {
                end_loc.add_line();
            } else {
                end_loc = end_loc + 1;
            }
        }

        // If we got at least min matches, we have a match
        if count >= self.min {
            ParseResult::matches(*loc, end_loc)
        } else {
            ParseResult::no_match()
        }
    }
}

impl<R: MatchStr> Display for UntilMatcher<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.min {
            0 => write!(f, "(!{})*", self.until),
            1 => write!(f, "(!{})+", self.until),
            _ => write!(f, "(!{}){{{},...}}", self.until, self.min),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{ParseInfo, Span, StrMatcher, StringCharReader};

    use super::*;

    #[test]
    fn test_until_matcher() {
        let rule = UntilMatcher::new(Rc::new(StrMatcher::new("a")), 1);

        let mut reader = StringCharReader::new("hello, a world");

        // Should match until the a
        let loc = Location::beginning();
        let info = ParseInfo::new(Span::new(loc, loc + 7), 7);
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), Some(info));

        let mut reader = StringCharReader::new("hello, world");

        // Should match until the end
        let loc = Location::beginning();
        let info = ParseInfo::new(Span::new(loc, loc + 12), 12);
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), Some(info));

        let mut reader = StringCharReader::new("a world");

        // Should not match
        let loc = Location::beginning();
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), None);

        // Should match empty string
        let rule2 = UntilMatcher::new(Rc::new(StrMatcher::new("a")), 0);
        let loc = Location::beginning();
        let info = ParseInfo::new(Span::new(loc, loc), 0);
        assert_eq!(rule2.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule2.test(&loc, &mut reader).unwrap(), Some(info));
        
        // Should match 1 char
        let mut reader = StringCharReader::new(" a world");
        let loc = Location::beginning();
        let info = ParseInfo::new(Span::new(loc, loc + 1), 1);
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), Some(info));

    }
}
