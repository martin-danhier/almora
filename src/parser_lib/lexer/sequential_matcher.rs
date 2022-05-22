use std::{fmt::Display, rc::Rc};

use crate::parser_lib::{CreateParseResult, Location, MatchStr, MatchToken, ParseResult};

/// Matcher that returns true if the given matcher matches the string, or not
#[derive(Debug)]
pub struct SequentialMatcher<R: MatchStr> {
    children: Vec<Rc<dyn MatchToken<R>>>,
}

impl<R: MatchStr> SequentialMatcher<R> {
    pub fn new(children: Vec<Rc<dyn MatchToken<R>>>) -> Self {
        Self { children }
    }
}

impl<R: MatchStr> MatchToken<R> for SequentialMatcher<R> {
    fn test(&self, loc: &Location, reader: &mut R) -> ParseResult {
        let mut end_loc = *loc;

        // Try to match each child
        for child in &self.children {
            if let Some(res) = child.test(&end_loc, reader)? {
                // If the child matched, update the end location
                end_loc = *res.span().end();
            } else {
                // None: one of the children didn't match, thus the whole sequence doesn't match
                // We can stop here
                return ParseResult::no_match();
            }
        }

        // If we get here, we have either a full match, or an empty match (if there is no children)
        ParseResult::matches(*loc, end_loc)
    }
}

impl<R: MatchStr> Display for SequentialMatcher<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Simply write children one after another
        write!(
            f,
            "({})",
            self.children
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{OptionalMatcher, ParseInfo, Span, StrMatcher, StringCharReader};

    use super::*;

    #[test]
    fn test_sequential_matcher() {
        let rule = SequentialMatcher::new(vec![
            Rc::new(StrMatcher::new("hello ")),
            Rc::new(StrMatcher::new("world")),
        ]);

        let mut reader = StringCharReader::new("hello world");

        // Test rule
        let info = ParseInfo::new(
            Span::new(Location::beginning(), Location::new(1, 12, 11)),
            11,
        );
        let loc = Location::beginning();
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), Some(info));

        let loc2 = loc + 1;
        assert_eq!(rule.test(&loc2, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc2, &mut reader).unwrap(), None);

        // Now, try with a different input string
        let mut reader = StringCharReader::new("hello how are you?");
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), None);

        // Let's try to combine it with optional
        let rule2 = SequentialMatcher::new(vec![
            Rc::new(OptionalMatcher::new(Rc::new(StrMatcher::new("hello ")))),
            Rc::new(StrMatcher::new("world")),
        ]);

        let mut reader = StringCharReader::new("hello world");

        // Should be able to match the whole string
        let info = ParseInfo::new(
            Span::new(Location::beginning(), Location::new(1, 12, 11)),
            11,
        );
        assert_eq!(rule2.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule2.test(&loc, &mut reader).unwrap(), Some(info));

        // Should also be able to just match the end
        let loc3 = loc + 6;
        let info = ParseInfo::new(Span::new(loc3, Location::new(1, 12, 11)), 5);
        assert_eq!(rule2.test(&loc3, &mut reader).is_ok(), true);
        assert_eq!(rule2.test(&loc3, &mut reader).unwrap(), Some(info));

        let mut reader = StringCharReader::new("world news");
        let info = ParseInfo::new(Span::new(loc, Location::new(1, 6, 5)), 5);
        assert_eq!(rule2.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule2.test(&loc, &mut reader).unwrap(), Some(info));

        // But just the beginning won't work
        let mut reader = StringCharReader::new("hello how are you?");
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), None);
    }
}
