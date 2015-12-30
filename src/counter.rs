use std::io;
use std::cmp::max;

pub struct Count {
    pub newlines: u32,
    pub words: u32,
    pub bytes: u32,
    pub chars: u32,
    pub max_line_length: u32,
}

impl Count {
    fn new() -> Self {
        Count {
            newlines: 0,
            words: 0,
            bytes: 0,
            chars: 0,
            max_line_length: 0,
        }
    }

    /// Generate newline, word, character, byte, and maximum line length counts for the given
    /// iterator over a set of bytes. A word is a non-zero-length sequence of characters delimited
    /// by white space.
    pub fn count<I>(bytes: I) -> io::Result<Count>
        where I: Iterator<Item=io::Result<u8>>
    {
        enum State {
            Whitespace,
            Word,
        }

        let mut count = Count::new();
        let mut current_line_length = 0;

        let mut state = State::Whitespace;
        for c in bytes {
            count.bytes += 1;
            count.chars += 1; // XXX what is a char?
            let c = try!(c) as char;
            match c {
                '\n' => {
                    count.newlines += 1;
                    current_line_length = 0;
                }
                _ => {
                    current_line_length += 1;
                    count.max_line_length = max(count.max_line_length, current_line_length);
                }
            }

            state = match state {
                State::Whitespace if !c.is_whitespace() => {
                    count.words += 1;
                    State::Word
                }
                State::Word if c.is_whitespace() => State::Whitespace,
                state => state
            }

        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use super::Count;

    fn vec_from_string(s: &str) -> Vec<io::Result<u8>> {
        s.bytes()
            .map(|c| Ok(c))
            .collect()
    }

    #[test]
    fn one_word() {
        let count = Count::count(vec_from_string("word\n").into_iter()).unwrap();
        assert_eq!(count.newlines, 1);
        assert_eq!(count.words, 1);
        assert_eq!(count.bytes, 5);
        assert_eq!(count.chars, 5);
        assert_eq!(count.max_line_length, 4);
    }

    #[test]
    fn one_word_no_newline() {
        let count = Count::count(vec_from_string("word").into_iter()).unwrap();
        assert_eq!(count.newlines, 0);
        assert_eq!(count.words, 1);
        assert_eq!(count.bytes, 4);
        assert_eq!(count.chars, 4);
        assert_eq!(count.max_line_length, 4);
    }
}
