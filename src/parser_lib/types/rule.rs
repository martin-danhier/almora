use std::{fmt::Display, rc::Rc};

use crate::parser_lib::{
    OptionalMatcher, RangeMatcher, RepetitionMatcher, SequentialMatcher, StrMatcher, ChoiceMatcher,
};

use super::{Location, MatchStr, MatchToken, ParseResult};

/// A "Rule" wraps a Matcher and gives it helper functions for clearer grammar definition.
#[derive(Debug)]
pub struct Rule<R: MatchStr> {
    matcher: Rc<dyn MatchToken<R>>,
}

impl<R: MatchStr> Display for Rule<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.matcher)
    }
}

impl<R: MatchStr> MatchToken<R> for Rule<R> {
    // Allow use as matcher, simply transfer the call to the underlying matcher
    fn test(&self, loc: &Location, reader: &mut R) -> ParseResult {
        self.matcher.test(loc, reader)
    }
}

impl<R: 'static + MatchStr> Rule<R> {
    /// Creates a new Rule from a Matcher.
    pub fn new(matcher: Rc<dyn MatchToken<R>>) -> Self {
        Self { matcher }
    }

    /// Matches an exact string.
    pub fn word(word: &'static str) -> Self {
        Self::new(Rc::new(StrMatcher::new(word)))
    }

    /// Matches characters within a range.
    pub fn range(start: char, end: char) -> Self {
        Self::new(Rc::new(RangeMatcher::new(start, end)))
    }

    /// Matches a sequence of rules.
    pub fn seq(rules: Vec<&Self>) -> Self {
        // Get all underlying matchers
        let matchers = rules.into_iter().map(|r| r.matcher.clone()).collect();

        // Create a sequential matcher
        Self::new(Rc::new(SequentialMatcher::new(matchers)))
    }

    /// Chooses between several rules.
    pub fn choice(rules: Vec<&Self>) -> Self {
        // Get all underlying matchers
        let matchers = rules.into_iter().map(|r| r.matcher.clone()).collect();

        // Create a sequential matcher
        Self::new(Rc::new(ChoiceMatcher::new(matchers)))
    }

    /// Repeats the rule at least n time.
    pub fn at_least(&self, n: u8) -> Self {
        let repeat = RepetitionMatcher::new(self.matcher.clone(), n);
        Self {
            matcher: Rc::new(repeat),
        }
    }

    /// Makes the rule optional.
    pub fn optional(&self) -> Self {
        let optional = OptionalMatcher::new(self.matcher.clone());
        Self {
            matcher: Rc::new(optional),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{Location, ParseInfo, Span, StringCharReader};

    use super::*;

    #[test]
    fn test_rule() {
        // Some fancy grammar can already be defined:
        let x = Rule::word("X");
        let y: Rule<StringCharReader> = Rule::word("Y");
        let val: Rule<StringCharReader> = Rule::choice(vec![&x, &y]);
        let space = Rule::word(" ");
        let comma = Rule::word(",");
        let ws = space.at_least(0);
        let param = Rule::seq(vec![&val, &ws]);
        let comma_ws = Rule::seq(vec![&comma, &ws]);
        let second_params = Rule::seq(vec![&comma_ws, &param]).at_least(0);
        let params = Rule::seq(vec![&param, &second_params]);

        // Same test as in the repetition matcher, but with rules instead of pure Matchers
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

        let mut reader = StringCharReader::new("X, Y, X");

        // Test rule
        let loc = Location::beginning();
        let info = ParseInfo::new(Span::new(loc, Location::new(1, 8, 7)), 7);
        assert_eq!(params.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(params.test(&loc, &mut reader).unwrap(), Some(info));

        let loc2 = loc + 3;
        let info2 = ParseInfo::new(Span::new(loc2, Location::new(1, 8, 7)), 4);
        assert_eq!(params.test(&loc2, &mut reader).is_ok(), true);
        assert_eq!(params.test(&loc2, &mut reader).unwrap(), Some(info2));
    }
}
