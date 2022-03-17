use std::{fmt::Display, rc::Rc};

use crate::parser_lib::{MatchToken, MatchStr, ParseResult, CreateParseResult, Location};

/// Matcher that returns true if the given matcher matches the string min times, or more
#[derive(Debug)]
pub struct RepetitionMatcher<R: MatchStr> {
    value: Rc<dyn MatchToken<R>>,
    min: u8,
}

impl<R: MatchStr> RepetitionMatcher<R> {
    pub fn new(value: Rc<dyn MatchToken<R>>, min: u8) -> Self {
        Self {
            value,
            min,
        }
    }
}

impl<R: MatchStr> MatchToken<R> for RepetitionMatcher<R> {
    fn test(&self, loc: &Location, reader: &mut R) -> ParseResult {
        let mut count = 0;
        let mut end_loc = *loc;

        // Try to match the matcher at the end until it doesn't work
        while let Ok(Some(res)) = self.value.test(&end_loc, reader) {
            // We got one more match
            count += 1;

            // The end location is thus further
            end_loc = *res.end();
        }

        // If we got at least min matches, we have a match
        if count >= self.min {
            ParseResult::matches(*loc, end_loc)
        }
        else {
            ParseResult::no_match()
        }
    }
}

impl<R: MatchStr> Display for RepetitionMatcher<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.min {
            0 => write!(f, "{}*", self.value),
            1 => write!(f, "{}+", self.value),
            _ => write!(f, "{}{{{},...}}", self.value, self.min),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{StringCharReader, StrMatcher, ParseInfo, Span, SequentialMatcher};

    use super::*;

    #[test]
    fn test_repetition_matcher() {
        let rule = RepetitionMatcher::new(Rc::new(StrMatcher::new("a")), 1);

        let mut reader = StringCharReader::new("aaaallo");

        // Test rule
        let loc = Location::beginning();
        let info = ParseInfo::new(Span::new(loc, Location::new(1, 5, 4)), 4);
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), Some(info));

        // It should match less if it starts later
        let loc2 = loc + 1;
        let info2 = ParseInfo::new(Span::new(loc2, Location::new(1, 5, 4)), 3);
        assert_eq!(rule.test(&loc2, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc2, &mut reader).unwrap(), Some(info2));

        // But since min is 1, it should not match
        let mut reader = StringCharReader::new("hello");
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), None);

        let rule = RepetitionMatcher::new(Rc::new(StrMatcher::new("a")), 0);

        // If we modify the rule to have a min 0, it should match
        let info2 = ParseInfo::new(Span::new(loc, loc), 0);
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), Some(info2));

        let rule = RepetitionMatcher::new(Rc::new(StrMatcher::new("aa")), 2);

        let mut reader = StringCharReader::new("aaaaallo");

        // Min can also be greater than 1, and string matcher can be greater as well. Here, we should match the same as first time
        let info3 = ParseInfo::new(Span::new(loc, Location::new(1, 5, 4)), 4);
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), Some(info3));

        let mut reader = StringCharReader::new("aaallo");

        // But if we have a string that is smaller than min, it should not match
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), None);
    }

    #[test]
    fn test_list() {
        // Some fancy grammar can already be defined:
        let x = Rc::new(StrMatcher::new("X"));
        let space = Rc::new(StrMatcher::new(" "));
        let comma = Rc::new(StrMatcher::new(","));
        let ws = Rc::new(RepetitionMatcher::new(space, 0));
        let param = Rc::new(SequentialMatcher::new(vec![x, ws.clone()]));
        let comma_ws = Rc::new(SequentialMatcher::new(vec![comma, ws]));
        let second_param = Rc::new(SequentialMatcher::new(vec![comma_ws, param.clone()]));
        let second_params = Rc::new(RepetitionMatcher::new(second_param, 0));
        let params = Rc::new(SequentialMatcher::new(vec![param, second_params]));

        let mut reader = StringCharReader::new("X, X, X");

        // Test rule
        let loc = Location::beginning();
        let info = ParseInfo::new(Span::new(loc, Location::new(1, 8, 7)), 7);
        assert_eq!(params.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(params.test(&loc, &mut reader).unwrap(), Some(info));

        // Should work starting from the second X
        let loc2 = loc + 3;
        let info2 = ParseInfo::new(Span::new(loc2, Location::new(1, 8, 7)), 4);
        assert_eq!(params.test(&loc2, &mut reader).is_ok(), true);
        assert_eq!(params.test(&loc2, &mut reader).unwrap(), Some(info2));

        let mut reader = StringCharReader::new("X  ,    X    ,    X");

        // It should ignore spaces
        let info3 = ParseInfo::new(Span::new(loc, Location::new(1, 20, 19)), 19);
        assert_eq!(params.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(params.test(&loc, &mut reader).unwrap(), Some(info3));

        // Even support when there is no space at all
        let mut reader = StringCharReader::new("X,X,X");
        let info4 = ParseInfo::new(Span::new(loc, Location::new(1, 6, 5)), 5);
        assert_eq!(params.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(params.test(&loc, &mut reader).unwrap(), Some(info4));

        // But if there is no comma, it should just match the first X
        let mut reader = StringCharReader::new("X X X");
        let info5 = ParseInfo::new(Span::new(loc, Location::new(1, 3, 2)), 2);
        assert_eq!(params.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(params.test(&loc, &mut reader).unwrap(), Some(info5));
    }
}