use crate::parser_lib::MatchStr;

use super::grammar::*;

pub fn compile<R: 'static + MatchStr>() {
    let grammar = almora::define_grammar::<R>();
    println!("{}", grammar);
}

#[cfg(test)]
mod tests {
    use crate::parser_lib::{Location, MatchToken, ParseInfo, Span, StringCharReader};

    use super::*;

    #[test]
    fn test_compile() {}
}
