use crate::{choice, define_grammar, seq, until, word};

define_grammar!(almora, |grammar: &mut GrammarBuilder<R>| {
    // ===== Config ignore list =====
    let line_comment = seq!(word!("//"), until!(word!("\n"), 0), word!("\n"));
    let block_comment = seq!(word!("/*"), until!(word!("*/"), 0), word!("*/"));
    let whitespace = choice![word!(" "), word!("\t"), word!("\n"), word!("\r")];
    let ignore = choice![line_comment, block_comment, whitespace];
    // grammar.ignore(ignore);

    

    // Save the root rule.
    ignore
});


#[cfg(test)]
mod tests {
    use crate::parser_lib::{StringCharReader, MatchToken, Location};

    use super::*;

    #[test]
    fn test_compile() {
        let almora_grammar = almora::define_grammar();

        let mut matcher = StringCharReader::new("/* hey */a");

        // Parse the input.
        let loc = Location::beginning();
        let result = almora_grammar.test(&loc, &mut matcher);
        assert_eq!(result.is_ok(), true);

        println!("{:?}", result);
    }
}