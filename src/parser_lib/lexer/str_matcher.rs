use std::fmt::Display;

use crate::parser_lib::{CreateParseResult, Location, MatchStr, MatchToken, ParseResult, Span, Stream};

/// Matcher that tries to match an exact string (like a keyword).
#[derive(Debug)]
pub struct StrMatcher {
    value: &'static str,

    // Information about the size of the value
    // When the value is matched, delta_lines will be added to the number of lines
    // And delta_columns will be added to the number of columns.
    // If a new line occurs, the columns will be reset to 1 before adding delta_columns.
    delta_lines: usize,
    delta_columns: usize,
}

impl StrMatcher {
    pub fn new(value: &'static str) -> Self {
        // Measure delta lines and delta column only once
        // Then we will be able to use those at each match instead
        // of having to recompute it again
        let mut delta_lines = 0;
        let mut delta_columns = 0;
        for c in value.chars() {
            if c == '\n' {
                delta_lines += 1;
                delta_columns = 0;
            } else {
                delta_columns += 1;
            }
        }

        // Save the information
        Self {
            value,
            delta_lines,
            delta_columns,
        }
    }
}

impl<R: MatchStr + Stream<char>> MatchToken<R> for StrMatcher {
    fn test(&self, loc: &Location, reader: &mut R) -> ParseResult {
        // Test to see if the string is in the input at the given location
        let success = reader.match_str(loc.index(), self.value)?;

        if success {
            // If it worked, compute the span
            let end_loc = loc.add_delta(self.delta_lines, self.delta_columns, self.value.len());
            let span = Span::new(*loc, end_loc);
            return ParseResult::new(span, self.value.len());
        }

        ParseResult::no_match()
    }
}

impl Display for StrMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", match self.value {
            "\n" => "\\n",
            "\r" => "\\r",
            "\t" => "\\t",
            "\0" => "\\0",
            "\"" => "\\\"",
            "\\" => "\\\\",
            v => v,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{ParseInfo, StringCharReader};

    use super::*;

    #[test]
    fn test_deltas() {
        // No new line
        let rule = StrMatcher::new("hello");
        assert_eq!(rule.delta_lines, 0);
        assert_eq!(rule.delta_columns, 5);

        // New line
        let rule = StrMatcher::new("\nhello");
        assert_eq!(rule.delta_lines, 1);
        assert_eq!(rule.delta_columns, 5);

        // New line in the middle
        let rule = StrMatcher::new("hello\nworld");
        assert_eq!(rule.delta_lines, 1);
        assert_eq!(rule.delta_columns, 5);

        // New line at the end
        let rule = StrMatcher::new("hello\n");
        assert_eq!(rule.delta_lines, 1);
        assert_eq!(rule.delta_columns, 0);

        // Empty string
        let rule = StrMatcher::new("");
        assert_eq!(rule.delta_lines, 0);
        assert_eq!(rule.delta_columns, 0);

        // String representation should be "\"hello\""
        let rule = StrMatcher::new("hello");
        assert_eq!(format!("{}", rule), "\"hello\"");
    }

    #[test]
    fn test_str_matcher() {
        let rule = StrMatcher::new("hello");
        let rule2 = StrMatcher::new("world");

        let mut reader = StringCharReader::new("hello world");

        // Rule 1
        let loc = Location::beginning();
        let info = ParseInfo::new(Span::new(loc, Location::new(1, 6, 5)), 5);
        assert_eq!(rule.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc, &mut reader).unwrap(), Some(info));

        let loc2 = loc + 1;
        assert_eq!(rule.test(&loc2, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc2, &mut reader).unwrap(), None);

        let loc3 = loc + 6;
        assert_eq!(rule.test(&loc3, &mut reader).is_ok(), true);
        assert_eq!(rule.test(&loc3, &mut reader).unwrap(), None);

        // Rule 2
        assert_eq!(rule2.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(rule2.test(&loc, &mut reader).unwrap(), None);

        let info2 = ParseInfo::new(Span::new(loc3, Location::new(1, 12, 11)), 5);
        assert_eq!(rule2.test(&loc3, &mut reader).is_ok(), true);
        assert_eq!(rule2.test(&loc3, &mut reader).unwrap(), Some(info2));
    }
}
