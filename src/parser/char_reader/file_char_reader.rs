use std::{error::Error, fs::File};

/// Char reader that streams characters from a file.
///
/// Maintains a buffer for peaked characters.
pub struct FileCharReader {
    /// The file to read from.
    file: File,
    /// The buffer of characters.
    buffer: Vec<u8>,
    /// The byte index of the next character to read in the file.
    file_index: usize,
    /// The byte index of the next character to peak.
    buffer_index: usize,
}

impl FileCharReader {
    /// Creates a new file char reader for the given file with the given buffer size
    pub fn new(filepath: &str, buffer_size: usize) -> Result<Self, Box<dyn Error>> {
        let file = File::open(filepath)?;

        Ok(FileCharReader {
            file,
            buffer: Vec::with_capacity(buffer_size),
            file_index: 0,
            buffer_index: 0,
        })
    }

    // Caches the given char in the buffer
}
