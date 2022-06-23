use crate::utils::Peek;

pub struct Lexer<R: Peek> {
    source: R,
}

impl<R: Peek> Lexer<R> {
    pub fn new(source: R) -> Self {
        Self { source }
    }
}







#[cfg(test)]
mod tests {
    use crate::parser_lib::input::{FileReader, StringReader};
    use std::fs::File;

    use super::*;

    #[test]
    fn test_string_lexer() {
        // Create lexer based on string
        let lexer = Lexer::new(StringReader::new("Hello"));
    }

    #[test]
    fn test_file_lexer() {
        // Create lexer based on file
        let file = File::open("resources/test_files/test.txt").unwrap();
        let reader = FileReader::new(file);
        let lexer = Lexer::new(reader);
    }
}
