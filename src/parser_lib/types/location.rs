use std::{fmt::Display, ops::Add};

/// Location of a point in a source file.
///
/// The location is defined by the line and column number.
///
/// Both numbers are 1-based, so the start of the file is (1, 1).
///
/// - Adding a ``usize`` to a ``Location`` increments the column number.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Location {
    line: usize,
    column: usize,
    index: usize,
}

impl Location {
    pub fn new(line: usize, column: usize, index: usize) -> Self {
        Self {
            line,
            column,
            index,
        }
    }

    /// Returns a position which is the beginning of a file
    #[allow(unused)]
    pub fn beginning() -> Self {
        Self::new(1, 1, 0)
    }

    #[allow(unused)]
    pub fn line(&self) -> usize {
        self.line
    }

    #[allow(unused)]
    pub fn column(&self) -> usize {
        self.column
    }

    pub fn index(&self) -> usize {
        self.index
    }

    #[allow(unused)]
    pub fn add_line(&self) -> Self {
        Self {
            line: self.line + 1,
            column: 1, // Columns are still 1-based
            index: self.index + 1,
        }
    }

    /// Increments the location according to the given char.
    ///
    /// The increment is done **in place**.
    #[allow(unused)]
    pub fn increment_for(&mut self, c: char) {
        match c {
            '\n' => {
                self.line += 1;
                self.index += 1;
                self.column = 1;
            }
            _ => {
                self.column += 1;
                self.index += 1;
            }
        }
    }

    pub fn add_delta(&self, delta_lines: usize, delta_columns: usize, delta_index: usize) -> Self {
        let index = self.index + delta_index;
        let line = self.line + delta_lines;

        // If there is a new line, the column is reset to 1
        let column = if delta_lines > 0 { 1 } else { self.column } + delta_columns;

        Self::new(line, column, index)
    }
}

// Operator overloading for convenience
// Add a usize to a location: we don't have any new line, so add just columns
impl Add<usize> for Location {
    type Output = Self;

    fn add(self, nb: usize) -> Self {
        Self {
            line: self.line,
            column: self.column + nb,
            index: self.index + nb,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location() {
        let mut loc = Location::new(1, 2, 1);
        assert_eq!(loc.line(), 1);
        assert_eq!(loc.column(), 2);
        assert_eq!(loc.index(), 1);

        let loc2 = loc + 3;
        assert_eq!(loc2.line(), 1);
        assert_eq!(loc2.column(), 5);
        assert_eq!(loc2.index(), 4);

        let loc4 = loc.add_line();
        assert_eq!(loc4.line(), 2);
        assert_eq!(loc4.column(), 1);
        assert_eq!(loc4.index(), 2);

        loc.increment_for('\n');
        assert_eq!(loc.line(), 2);
        assert_eq!(loc.column(), 1);
        assert_eq!(loc.index(), 2);

        loc.increment_for('a');
        assert_eq!(loc.line(), 2);
        assert_eq!(loc.column(), 2);
        assert_eq!(loc.index(), 3);
    }
}
