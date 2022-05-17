use std::{fmt::Display, rc::Rc};

use crate::parser_lib::{CreateParseResult, Location, MatchStr, MatchToken, ParseResult};

/// Matcher that returns true if the next char is in the given range
/// Avoids to check individually every possibility if the binary range is continuous.
///
/// - start: inclusive start of the range
/// - end: inclusive end of the range
///
/// Note: does not support new lines in the range (won't update the location accordingly)
/// They are not supported because they are not in an useful range anyway. Use a choice matcher instead.
#[derive(Debug)]
pub struct RangeMatcher {
    start: char,
    end: char,
    /// Min number of matching chars
    min: u8,
    /// Max number of matching chars
    /// If 0, considered as infinite
    max: u8
}

impl RangeMatcher {
    /// Create matcher for a single char in range
    pub fn new(start: char, end: char) -> Self {
        Self {
            start,
            end,
            min: 1,
            max: 1
        }
    }

    /// Create matcher for a range of chars, with a minimum number of matching chars and infinite max
    pub fn at_least_n(start: char, end: char, min: u8) -> Self {
        Self {
            start,
            end,
            min,
            max: 0
        }
    }

    /// Create matcher for a range of chars, with a minimum and maximum number of matching chars
    pub fn repeat_between(start: char, end: char, min: u8, max: u8) -> Self {
        Self {
            start,
            end,
            min,
            max
        }
    }
}

impl<R: MatchStr> MatchToken<R> for RangeMatcher {
    fn test(&self, loc: &Location, reader: &mut R) -> ParseResult {
        // Test to see if the string is in the input at the given location
        let nb = reader.match_range(loc.index(), self.start, self.end, self.max)?;

        if nb >= self.min.into() {
            // If it worked, compute the span
            return ParseResult::matches(*loc, *loc + nb.try_into().unwrap());
        }

        ParseResult::no_match()
    }
}

impl Display for RangeMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}-{}]", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::min;

    use crate::parser_lib::{ParseInfo, Span, StringCharReader};

    use super::*;

    #[test]
    fn test_range_match() {
        let rule = RangeMatcher::new('a', 'z');

        let mut reader = StringCharReader::new("abcdefghijklmnopqrstuvwxyzA");

        let mut loc = Location::beginning();

        // They must all succeed
        for _ in 0..26 {
            let info = ParseInfo::new(Span::new(loc, loc + 1), 1);
            assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
            assert_eq!(rule.test(&loc, &mut reader).unwrap(), Some(info));

            // Increment loc
            loc = loc + 1;
        }

        // But the next one must fail
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), None);

        // String representation should be "[a-z]"
        assert_eq!(rule.to_string(), "[a-z]");
    }

    #[test]
    fn test_at_least_range() {
        let rule = RangeMatcher::at_least_n('a', 'z', 5);

        let mut reader = StringCharReader::new("abcdefghijklmnopqrstuvwxyzA");

        let mut loc = Location::beginning();
        let end_loc = loc + 26;

        // They must all succeed and match the whole range starting from loc, except the last 5 because of the min
        for i in 0..22 {
            let info = ParseInfo::new(Span::new(loc, end_loc), 26 - i);
            assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
            assert_eq!(rule.test(&loc, &mut reader).unwrap(), Some(info));

            // Increment loc
            loc = loc + 1;
        }

        // But the next one must fail
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), None);
    }

    #[test]
    fn test_between_range() {
        let rule = RangeMatcher::repeat_between('a', 'z', 5, 10);

        let mut reader = StringCharReader::new("abcdefghijklmnopqrstuvwxyzA");

        let mut loc = Location::beginning();

        // They must all succeed and match the whole range starting from loc, except the last 5 because of the min
        // For the max, the first ones will be 10 char longs, but the last one will be less
        for i in 0..22 {
            let size = min(26 - i, 10);
            let info = ParseInfo::new(Span::new(loc, loc + size), size);
            assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
            assert_eq!(rule.test(&loc, &mut reader).unwrap(), Some(info));

            // Increment loc
            loc = loc + 1;
        }

        // But the next one must fail
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), None);
    }
}
