use std::env;
use std::process;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::cmp::max;
extern crate rust_wc;
use rust_wc::counter::Count;

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

    let mut count = Count::new();
    let mut current_line_length = 0;

    let mut state = State::Whitespace;
    let reader = BufReader::new(file);
    for c in reader.bytes() {
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

        // Print newline, word, and byte counts for each FILE, and a total line if
        // more than one FILE is specified.  With no FILE, or when FILE is -, read
        // standard  input.   A  word  is a non-zero-length sequence of characters
        // delimited by white space.  The options below  may  be  used  to  select
        // which counts are printed, always in the following order: newline, word,
        // character, byte, maximum line length.
    }

    // XXX the formatting is such that each field takes up the space of the longest output field
    println!("{} {} {} {} {} {:?}", count.newlines, count.words, count.chars, count.bytes, count.max_line_length, path);
}
