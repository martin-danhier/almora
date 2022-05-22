use std::{fmt::Display, rc::Rc};

use crate::parser_lib::{CreateParseResult, Location, MatchStr, MatchToken, ParseResult};

/// Matcher that tries to match one of the given matchers
#[derive(Debug)]
pub struct ChoiceMatcher<R: MatchStr> {
    children: Vec<Rc<dyn MatchToken<R>>>,
}

impl<R: MatchStr> ChoiceMatcher<R> {
    pub fn new(children: Vec<Rc<dyn MatchToken<R>>>) -> Self {
        Self { children }
    }
}

impl<R: MatchStr> MatchToken<R> for ChoiceMatcher<R> {
    fn test(&self, loc: &Location, reader: &mut R) -> ParseResult {
        // Try to match the first child. If it doesn't work, start from the beginning and try the second, and so on.
        for child in &self.children {
            if let Some(res) = child.test(loc, reader)? {
                return ParseResult::matches(*loc, *res.span().end());
            }
        }

        ParseResult::no_match()
    }
}

impl<R: MatchStr> Display for ChoiceMatcher<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write children seperated by "|"
        write!(
            f,
            "({})",
            self.children
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<_>>()
                .join(" | ")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{ParseInfo, Span, StrMatcher, StringCharReader};

    use super::*;

    #[test]
    fn test_choice_matcher() {
        let rule = ChoiceMatcher::new(vec![
            Rc::new(StrMatcher::new("hey ")),
            Rc::new(StrMatcher::new("world")),
        ]);

        // First matches but not the second
        let mut reader = StringCharReader::new("hey you");

        let info = ParseInfo::new(Span::new(Location::beginning(), Location::new(1, 5, 4)), 4);
        let loc = Location::beginning();
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), Some(info));

        // Second matches but not the first
        reader = StringCharReader::new("world you");

        let info = ParseInfo::new(Span::new(Location::beginning(), Location::new(1, 6, 5)), 5);
        let loc = Location::beginning();
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), Some(info));

        // None match
        reader = StringCharReader::new("hello you");

        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), None);

        // If both are one after the other, it should only match the first (its not a repetition, just a choice)
        reader = StringCharReader::new("hey world");

        let info = ParseInfo::new(Span::new(Location::beginning(), Location::new(1, 5, 4)), 4);
        let loc = Location::beginning();
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), Some(info));

        // String representation should be "(hey |world)"
        assert_eq!(format!("{}", rule), "(\"hey \" | \"world\")");
    }
}
