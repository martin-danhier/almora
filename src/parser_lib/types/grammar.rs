use std::fmt::{Display, Formatter, Error};

use super::{MatchStr, Rule, MatchToken, ParseResult, CreateParseResult, Location, ParserError};
use crate::word;

#[derive(Debug)]
pub struct Grammar<R: MatchStr> {
    /// Root rule of the grammar.
    ///
    /// The intermediate rules are not needed, everything is stored in the root rule.
    root: Option<Rule<R>>,
    /// Keywords that are not allowed for identifiers.
    reserved_words: Vec<String>,
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

impl<R: 'static + MatchStr> GrammarBuilder<R> {
    pub fn new() -> Self {
        let grammar = Grammar::<R> {
            root: None,
            reserved_words: Vec::new(),
        };
        GrammarBuilder { grammar }
    }

    pub fn reserved(&mut self, word: &'static str) -> Rule<R> {
        self.grammar.reserved_words.push(word.to_string());
        word!(word)
    }

    pub fn save_root(mut self, root: Rule<R>) -> Grammar<R> {
        self.grammar.root = Some(root);
        self.grammar
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

            // Create the function
            #[allow(dead_code)]
            pub fn define_grammar<R: 'static + MatchStr>() -> Grammar<R> {
                let mut builder = GrammarBuilder::<R>::new();

                let root: Rule<R> = $body(&mut builder);
                builder.save_root(root)
            }
        }
    };
}