use std::rc::Rc;

use crate::parser_lib::{MatchStr, Stream, MatchToken};

pub struct Tokenizer<R: MatchStr> {
    matchers: Vec<Rc<dyn MatchToken<R>>>,
    reader: R,
}