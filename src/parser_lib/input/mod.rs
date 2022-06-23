/// # Input module
///
/// This modules aims to provide a unified interface to read input characters.
/// Characters can come from a file, a string, ...

mod file_reader;
mod try_into_char;
mod string_reader;

pub use file_reader::FileReader;
pub use string_reader::StringReader;