pub trait ReadUTF8<'a> {
    /// Returns the UTF-8 character in the string that starts at the given byte index.
    /// 
    /// Such a character can measure up to 4 bytes in UTF-8.
    /// 
    /// The function returns an option:
    /// - if a char is found, it is returned, as well as its size (up to 4 bytes)
    /// - if the index is invalid, or no char is found, None is returned
    /// 
    /// Be careful:
    /// - the index is the **byte index** and not the UTF char index.
    ///   For example, if the string is "éléphant", the index of "l" is 2, not 1.
    /// - the function works if the given index is at the start of a utf8 char. If not, it will return None.
    fn get_utf8(&'a self, start_index: usize) -> Option<(&'a str, usize)>;
}

impl<'a> ReadUTF8<'a> for String {
    fn get_utf8(&'a self, start_index: usize) ->  Option<(&'a str, usize)> {
        let mut index = start_index + 1;

        // Try to consume more and more characters to finally have a valid utf8 char
        // Also don't search if there are more than 4 chars in the slice: its the max of unicode
        // it would mean that the first index is at the middle of a char
        while index <= self.len() && index - start_index <= 4 {
            let res = self.get(start_index..index);

            if let Some(c) = res {
                // Found a valid char !
                return Some((c, index - start_index));
            }
            else {
                // Didn't found a char, increment the index and try a bigger slice
                index += 1;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_utf8() {
        let s = String::from("e😎 éléphant");

        // The first one should work
        let res = s.get_utf8(0);
        assert_eq!(res, Some(("e", 1)));

        // Should be able to get even large unicode slices
        let res = s.get_utf8(1);
        assert_eq!(res, Some(("😎", 4)));

        // Should be able to get unicode chars
        let res = s.get_utf8(6);
        assert_eq!(res, Some(("é", 2)));

        // But also ascii chars
        let res = s.get_utf8(8);
        assert_eq!(res, Some(("l", 1)));

        // Fetching the last char should work
        let res = s.get_utf8(15);
        assert_eq!(res, Some(("t", 1)));

        // Fetching after the end should return None
        let res = s.get_utf8(16);
        assert_eq!(res, None);

        // Fetching with an invalid index (like the middle of a char) should return None
        let res = s.get_utf8(2);
        assert_eq!(res, None);
    }
}