use crate::parser_lib::MatchStr;

use super::grammar::*;

pub fn compile<R: 'static + MatchStr> () {
    let grammar = almora::define_grammar::<R>();
    println!("{}", grammar);
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{StringCharReader, ParseInfo, Span, Location, MatchToken};

    use super::*;

    #[test]
    fn test_compile() {
        let almora_grammar = almora::define_grammar::<StringCharReader>();

        let mut reader = StringCharReader::new("22+13");

        // It should match everything
        let info = ParseInfo::new(Span::new(Location::beginning(), Location::new(1, 6, 5)), 5);
        let loc = Location::beginning();
        assert_eq!(almora_grammar.test(&loc, &mut reader).is_ok(), true);
        assert_eq!(almora_grammar.test(&loc, &mut reader).unwrap(), Some(info));
    }
}