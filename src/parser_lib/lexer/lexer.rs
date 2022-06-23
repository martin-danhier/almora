use crate::utils::Peek;

pub struct Lexer<R: Peek> {
    source: R,
}

impl<R: Peek<Item = char>> Lexer<R> {
    pub fn new(source: R) -> Self {
        Self { source }
    }

    /// Try to match the given string to the input. Returns true if it did. Doesn't consume anything
    fn match_str(&mut self, string: &'static str) -> bool {
        for (i, tested_c) in string.chars().enumerate() {
            // Check if the character is the same in the input
            let matches = match self.source.peek_nth(i) {
                Some(c) => c == tested_c,
                // EOF or too far lookahead: not a match since we expected a char
                None => false,
            };

            if !matches {
                // Stop here
                return false;
            }
        }

        // If we reach this point, the whole string matched
        true
    }

    /// Try to greedily match an identifier to the input. Returns it if it did, None otherwise.
    ///
    /// An identifier is equivalent to [a-zA-Z_][a-zA-Z0-9_]*
    // fn match_identifier(&mut self) -> Option<String> {
    //     for ()




    //     None
    // }

    fn parse_token() {}
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::input::{FileReader, StringReader};
    use std::fs::File;

    use super::*;

    // #[test]
    // fn test_string_lexer() {
    //     // Create lexer based on string
    //     let mut lexer = Lexer::new(StringReader::new("Hello"));

    //     assert!(lexer.match_str("Hell"));
    //     assert!(lexer.match_str("Hello"));
    //     assert!(lexer.match_str("H"));
    //     assert!(lexer.match_str(""));
    //     assert!(!lexer.match_str("Hello world"));
    //     assert!(!lexer.match_str("Hi"));
    //     assert!(!lexer.match_str("Bonjour"));
    //     assert!(!lexer.match_str("Haha"));
    // }

    // #[test]
    // fn test_file_lexer() {
    //     // Create lexer based on file
    //     let file = File::open("resources/test_files/test.txt").unwrap();
    //     let reader = FileReader::new(file);
    //     let lexer = Lexer::new(reader);
    // }
}
