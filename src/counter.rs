use std::io;
use std::cmp::max;

pub struct Count {
    pub newlines: u32,
    pub words: u32,
    pub bytes: u32,
    pub chars: u32,
    pub max_line: u32,
}

impl Count {
    fn new() -> Self {
        Count {
            newlines: 0,
            words: 0,
            bytes: 0,
            chars: 0,
            max_line: 0,
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
            // XXX what is a char? Keep a buffer that is 4 bytes long and use std::char::from_u32
            // to count characters. According to the internet, this is for counting unicode
            // characters.
            count.chars += 1;

            let c = try!(c) as char;
            if c == '\n' {
                count.newlines += 1;
                current_line_length = 0;
            }
            else {
                current_line_length += 1;
                count.max_line = max(count.max_line, current_line_length);
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
        assert_eq!(count.max_line, 4);
    }

    #[test]
    fn one_word_no_newline() {
        let count = Count::count(vec_from_string("word").into_iter()).unwrap();
        assert_eq!(count.newlines, 0);
        assert_eq!(count.words, 1);
        assert_eq!(count.bytes, 4);
        assert_eq!(count.chars, 4);
        assert_eq!(count.max_line, 4);
    }

    #[test]
    fn words_and_whitespace() {
        let count = Count::count(vec_from_string("   words and  \t\n  whitespace\n").into_iter()).unwrap();
        assert_eq!(count.newlines, 2);
        assert_eq!(count.words, 3);
        assert_eq!(count.bytes, 29);
        assert_eq!(count.chars, 29);
        assert_eq!(count.max_line, 15);
    }

    #[test]
    fn line_length() {
        let count = Count::count(vec_from_string(
r"Testing out some long lines to see if it picks the largest one correctly.
That last line was fairly long, but I think we can do better.
Apparentlly not.
Hahaha, just kidding! Of course we can do better. I can go on forever baby! Why don't we start with a list of my favorite movies. Back to the Future, The Last Dragon, Lock Stock and Two Smoking Barrels, Jurassic Park, Casablanca, Pulp Fiction, Forest Gump, City of God. I think that's enough.

And a short line to end it.
").into_iter()).unwrap();
        assert_eq!(count.newlines, 6);
        assert_eq!(count.max_line, 292);
    }
}
