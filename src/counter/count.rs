use std::io;
use std::io::prelude::*;
use std::io::{stdin, BufReader};
use std::fs::File;
use std::cmp::max;
use std::error::Error;
use super::display::Display;
use options::Options;

pub struct Count {
    pub newlines: u64,
    pub words: u64,
    pub bytes: u64,
    pub chars: u64,
    pub max_line: u64,
}

impl Count {
    pub fn new() -> Self {
        Count {
            newlines: 0,
            words: 0,
            bytes: 0,
            chars: 0,
            max_line: 0,
        }
    }

    /// Return a Count with only the number of bytes in the given file.
    pub fn bytes_from_file(file: &str) -> Result<Count, Box<Error>> {
        // read a single byte from the file to detect errors
        let mut buf = [0u8; 1];
        let mut file = try!(File::open(file));
        try!(file.read(&mut buf));

        let mut count = Self::new();
        count.bytes = try!(file.metadata()).len();
        Ok(count)
    }

    pub fn from_file(file: &str) -> Result<Count, Box<Error>> {
        let file = try!(File::open(file));
        let reader = BufReader::new(file);
        Count::from_iter(reader.bytes())
    }

    pub fn from_stdin() -> Result<Count, Box<Error>> {
        let stdin = stdin();
        let stdin = stdin.lock();
        let stdin = BufReader::new(stdin);
        let stdin = stdin.bytes();
        Count::from_iter(stdin)
    }

    /// Generate newline, word, character, byte, and maximum line length counts for the given
    /// iterator over a set of bytes. A word is a non-zero-length sequence of characters delimited
    /// by white space.
    fn from_iter<I>(bytes: I) -> Result<Count, Box<Error>>
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
            let c_byte = try!(c);
            count.bytes += 1;

            let c = c_byte as char;
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
            };

            // count utf8 single bytes and leading bytes, ignore continuation bytes
            if c_byte & 0b1100_0000 == 0b1000_0000 {
                // utf8 continuation byte, ignore it
            }
            else {
                // utf8 sigle byte or leading byte, count it
                count.chars += 1;
            }
        }

        Ok(count)
    }

    pub fn display<'a>(&'a self, opts: &'a Options) -> Display {
        Display::new(self, opts)
    }
}

use std::ops::Add;
impl Add for Count {
    type Output = Count;
    fn add(self, rhs: Self) -> Self::Output {
        Count {
            newlines: self.newlines + rhs.newlines,
            words: self.words + rhs.words,
            bytes: self.bytes + rhs.bytes,
            chars: self.chars + rhs.chars,
            max_line: self.max_line + rhs.max_line,
        }
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
        let count = Count::from_iter(vec_from_string("word\n").into_iter()).unwrap();
        assert_eq!(count.newlines, 1);
        assert_eq!(count.words, 1);
        assert_eq!(count.bytes, 5);
        assert_eq!(count.chars, 5);
        assert_eq!(count.max_line, 4);
    }

    #[test]
    fn one_word_no_newline() {
        let count = Count::from_iter(vec_from_string("word").into_iter()).unwrap();
        assert_eq!(count.newlines, 0);
        assert_eq!(count.words, 1);
        assert_eq!(count.bytes, 4);
        assert_eq!(count.chars, 4);
        assert_eq!(count.max_line, 4);
    }

    #[test]
    fn words_and_whitespace() {
        let count = Count::from_iter(vec_from_string("   words and  \t\n  whitespace\n").into_iter()).unwrap();
        assert_eq!(count.newlines, 2);
        assert_eq!(count.words, 3);
        assert_eq!(count.bytes, 29);
        assert_eq!(count.chars, 29);
        assert_eq!(count.max_line, 15);
    }

    #[test]
    fn line_length() {
        let count = Count::from_iter(vec_from_string(
r"Testing out some long lines to see if it picks the largest one correctly.
That last line was fairly long, but I think we can do better.
Apparentlly not.
Hahaha, just kidding! Of course we can do better. I can go on forever baby! Why don't we start with a list of my favorite movies. Back to the Future, The Last Dragon, Lock Stock and Two Smoking Barrels, Jurassic Park, Casablanca, Pulp Fiction, Forest Gump, City of God. I think that's enough.

And a short line to end it.
").into_iter()).unwrap();
        assert_eq!(count.newlines, 6);
        assert_eq!(count.max_line, 292);
    }

    #[test]
    fn unicode() {
        let count = Count::from_iter(vec_from_string("இঈஇ 💖\n").into_iter()).unwrap();
        assert_eq!(count.newlines, 1);
        assert_eq!(count.words, 2);
        assert_eq!(count.bytes, 15);
        assert_eq!(count.chars, 6);
        assert_eq!(count.max_line, 14);
    }

}
