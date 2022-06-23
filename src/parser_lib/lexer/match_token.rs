use crate::{utils::{Peek, CharRingBuffer}, parser_lib::input::FileReader};

pub trait MatchToken {
    fn match_str(&self, string: &'static str) -> bool;
}



