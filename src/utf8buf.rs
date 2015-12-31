use std::str::{from_utf8, Utf8Error};

/// A buffer that can hold a single 4 byte unicode character stored as utf8.
pub struct Utf8Buf {
    buf: [u8; 4],
    len: usize,
}

impl Utf8Buf {
    pub fn new() -> Self {
        Utf8Buf {
            buf: [0u8; 4],
            len: 0,
        }
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Returns `true` if the buffer contains 4 elements.
    pub fn is_full(&self) -> bool {
        self.len == 4
    }

    /// Add and item to the end of the bufer.
    ///
    /// # Failures
    /// This method will return false and do nothing if the buffer is already full (i.e. len() ==
    /// 4).
    pub fn push(&mut self, byte: u8) -> bool {
        if self.is_full() {
            // buffer is full
            return false;
        }

        self.buf[self.len] = byte;
        self.len += 1;
        true
    }

    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        from_utf8(&self.buf[..self.len])
    }
}

#[cfg(test)]
mod tests {
    use super::Utf8Buf;

    #[test]
    fn full_buff() {
        let mut buf = Utf8Buf::new();
        assert!(!buf.is_full());
        assert!(buf.push(0));
        assert!(buf.push(0));
        assert!(buf.push(0));
        assert!(buf.push(0));
        assert!(!buf.push(0));
        assert!(buf.is_full());
    }

    #[test]
    fn sparkle_heart_str() {
        let mut buf = Utf8Buf::new();
        buf.push(240);
        buf.push(159);
        buf.push(146);
        buf.push(150);

        match buf.to_str() {
            Ok(s) => assert_eq!(s, "💖"),
            _ => panic!("error converting utf8 to str"),
        }
    }

    #[test]
    fn a_str() {
        let mut buf = Utf8Buf::new();
        buf.push(97);

        match buf.to_str() {
            Ok(s) => assert_eq!(s, "a"),
            _ => panic!("error converting utf8 to str"),
        }
    }

    #[test]
    fn empty_str() {
        let mut buf = Utf8Buf::new();

        match buf.to_str() {
            Ok(s) => assert_eq!(s, ""),
            _ => panic!("error converting utf8 to str"),
        }

        buf.push(97);
        match buf.to_str() {
            Ok(s) => assert_eq!(s, "a"),
            _ => panic!("error converting utf8 to str"),
        }

        buf.clear();
        match buf.to_str() {
            Ok(s) => assert_eq!(s, ""),
            _ => panic!("error converting utf8 to str"),
        }
    }
}
