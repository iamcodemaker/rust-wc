use std::env;
use std::process;
use std::path::Path;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::cmp::max;
extern crate rust_wc;
use rust_wc::counter::Count;

/// Generate newline, word, character, byte, and maximum line length counts for the given iterator
/// over a set of bytes. A word is a non-zero-length sequence of characters delimited by white
/// space.
fn count<I>(bytes: I) -> Count
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
        let c = c.unwrap() as char;
        match c {
            '\n' => {
                count.newlines += 1;
                count.max_line_length = max(count.max_line_length, current_line_length);
                current_line_length = 0;
            }
            _ => current_line_length += 1,
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

    count
}

fn main() {
    if env::args().count() != 2 {
        println!("error: invalid arguments");
        process::exit(1);
    }

    let path = env::args().nth(1).unwrap();
    let path = Path::new(&path);

    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            println!("error opening file: {:?}", e);
            process::exit(1);
        }
    };

    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => {
            println!("error reading file metadata: {:?}", e);
            process::exit(1);
        }
    };

    if metadata.is_dir() {
        println!("{:?} is a directory", path);
        process::exit(1);
    }


    enum State {
        Whitespace,
        Word,
    }

    let reader = BufReader::new(file);
    let count = count(reader.bytes());


    // XXX the formatting is such that each field takes up the space of the longest output field
    println!("{} {} {} {} {} {:?}", count.newlines, count.words, count.chars, count.bytes, count.max_line_length, path);
}
