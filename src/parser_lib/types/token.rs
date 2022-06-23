use std::rc::Rc;

use super::{MatchStr, MatchToken, Span};

#[derive(PartialEq, Debug)]
pub struct Token<T: PartialEq> {
    span: Span,
    token_type: T,
}

impl<T: PartialEq> Token<T> {
    pub fn new(span: Span, token_type: T) -> Self {
        Self { span, token_type }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn token_type(&self) -> &T {
        &self.token_type
    }
}

#[derive(Debug)]
pub struct TokenType<R: MatchStr> {
    name: &'static str,
    matcher: Rc<dyn MatchToken<R>>,
}
impl<R: MatchStr> TokenType<R> {
    pub fn new(name: &'static str, matcher: Rc<dyn MatchToken<R>>) -> Self {
        Self { name, matcher }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn matcher(&self) -> &Rc<dyn MatchToken<R>> {
        &self.matcher
    }
}

macro_rules! define_tokens {
    ($R: ident, $($name: ident => $matcher: expr),*) => {
        mod tokens {
            use super::*;
            use crate::parser_lib::TokenType;
            use std::rc::Rc;

            // Aggregate all the token types in a vector for easy iteration
            pub const tokens: [TokenType<$R>; 2] = [$(TokenType::new(stringify!($name), Rc::new($matcher))),*];
        }
    };
}

// Define a macro to make this simpler
#[macro_export]
macro_rules! define_grammar_2 {
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
            pub fn define_grammar<R: 'static + MatchStr >() -> Grammar<R> {
                let mut builder = GrammarBuilder::<R>::new();

                let root: Rule<R> = $body(&mut builder);
                builder.save_root(root)
            }
        }
    };
}

macro_rules! separation {
    ($lang_name: ident, {
        tokens => { $($tok_name:  ident => $tok_matcher:  expr),* }
        rules  => { $($rule_name: ident => $rule_matcher: expr),* }
    }) => {

    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parser_lib::{Location, Rule, StrMatcher, StringCharReader},
        word,
    };

    use super::*;

    #[derive(PartialEq, Debug)]
    enum TestTokenType {
        TestTokenType1,
        TestTokenType2,
    }

    #[test]
    fn test_token() {
        let t1 = Token::new(
            Span::new(Location::beginning(), Location::beginning()),
            TestTokenType::TestTokenType1,
        );
        let t2 = Token::new(
            Span::new(Location::beginning(), Location::beginning()),
            TestTokenType::TestTokenType2,
        );

        // Not equal
        assert!(t1 != t2);
    }

    #[test]
    fn test_define() {
        // define_tokens! {
        //     StringCharReader,
        //     token1 => Rule::word("hello"),
        //     token2 => Rule::word("world")
        // }

        // println!("{:?}", tokens::tokens);

        separation! {
            almora, {
                tokens => {
                    token1 => word("hello"),
                    token2 => word("world"),
                }
                rules => {
                    rule1 => seq!(token1, token2)
                }
            }
        }
    }
}
