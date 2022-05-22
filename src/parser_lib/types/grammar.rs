use std::fmt::{Display, Error, Formatter};

use super::{CreateParseResult, Location, MatchStr, MatchToken, ParseResult, ParserError, Rule, Stream};
use crate::word;

#[derive(Debug)]
pub struct Grammar<R: MatchStr> {
    /// Root rule of the grammar.
    ///
    /// The intermediate rules are not needed, everything is stored in the root rule.
    root: Option<Rule<R>>,
    /// Keywords that are not allowed for identifiers.
    reserved_words: Vec<String>,
    ignored: Option<Rule<R>>,
}

impl<R: MatchStr> Display for Grammar<R> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match &self.root {
            Some(rule) => write!(f, "{}", rule),
            None => write!(f, "No grammar defined. Use `define_grammar!` macro."),
        }
    }
}

impl<R: MatchStr> MatchToken<R> for Grammar<R> {
    fn test(&self, loc: &Location, reader: &mut R) -> ParseResult {
        match &self.root {
            // Be sure to have a grammar
            None => ParseResult::error(ParserError::NoGrammarDefined),
            Some(rule) => rule.test(loc, reader),
        }
    }
}

#[derive(Debug)]
pub struct GrammarBuilder<R: MatchStr> {
    grammar: Grammar<R>,
}

impl<R: 'static + MatchStr + Stream<char>> GrammarBuilder<R> {
    pub fn new() -> Self {
        let grammar = Grammar::<R> {
            root: None,
            reserved_words: Vec::new(),
            ignored: None,
        };
        GrammarBuilder { grammar }
    }

    #[allow(unused)]
    pub fn reserved(&mut self, word: &'static str) -> Rule<R> {
        self.grammar.reserved_words.push(word.to_string());
        word!(word)
    }

    pub fn save_root(mut self, root: Rule<R>) -> Grammar<R> {
        self.grammar.root = Some(root);
        self.grammar
    }

    pub fn ignore(&mut self, ignored: Rule<R>) {
        self.grammar.ignored = Some(ignored);
    }
}

// Define a macro to make this simpler
#[macro_export]
macro_rules! define_grammar {
    // Take the language name and the body of the function, and
    // Create the impl and the function.
    // The function is named after the language with the _grammar suffix
    ($language:ident, $body:expr) => {
        pub mod $language {
            use super::*;
            use crate::parser_lib::Grammar;
            use crate::parser_lib::GrammarBuilder;
            use crate::parser_lib::MatchStr;
            use crate::parser_lib::Rule;
            use crate::parser_lib::Stream;

            // Create the function
            #[allow(unused)]
            pub fn define_grammar<R: 'static + MatchStr + Stream<char>>() -> Grammar<R> {
                let mut builder = GrammarBuilder::<R>::new();

                let root: Rule<R> = $body(&mut builder);
                builder.save_root(root)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        choice,
        parser_lib::{ParseInfo, Span, StringCharReader},
        range,
    };

    define_grammar!(my_grammar, |_grammar: &mut GrammarBuilder<R>| {
        // Basic tokens
        let plus = word!("+");
        let minus = word!("-");
        let times = word!("*");
        let divide = word!("/");
        let modulo = word!("%");

        let digit = range!('0', '9');
        let integer = digit.at_least(1);

        let operator = choice!(plus, minus, times, divide, modulo);

        let term = choice!(integer, operator);

        let expression = term.at_least(1);

        // Save the root rule.
        expression
    });

    #[test]
    fn test_grammar() {
        let grammar = my_grammar::define_grammar::<StringCharReader>();

        let mut reader = StringCharReader::new("22+13");

        // It should match everything
        let info = ParseInfo::new(Span::new(Location::beginning(), Location::new(1, 6, 5)), 5);
        let loc = Location::beginning();
        assert_eq!(grammar.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(grammar.test(&loc, &mut reader).unwrap(), Some(info));
    }
}
