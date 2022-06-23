pub trait TryIntoChar {
    type Error;

    fn try_into_char(self) -> Result<char, Self::Error>;
}

impl TryIntoChar for [u8; 4] {
    type Error = ();

    fn try_into_char(self) -> Result<char, Self::Error> {
        match std::str::from_utf8(&self) {
            Ok(s) => Ok(s.chars().next().unwrap()),
            Err(_) => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_into_char() {
        let mut char_buf = [0u8; 4];
        char_buf[0] = 240;
        char_buf[1] = 159;
        char_buf[2] = 152;
        char_buf[3] = 142;

        let c = char_buf.try_into_char().unwrap();

        assert_eq!(c, '😎');
    }
}