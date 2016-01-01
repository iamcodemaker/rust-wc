use std::process;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
extern crate rust_wc;
use rust_wc::counter::Count;
use rust_wc::options::Options;

fn main() {
    let opts = Options::new();

    if opts.files().count() != 1 {
        println!("error: invalid arguments");
        process::exit(1);
    }

    let path = opts.files().nth(0).unwrap();
    let path = Path::new(path);

    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            println!("error opening file: {}", e);
            process::exit(1);
        }
    };

    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => {
            println!("error reading file metadata: {}", e);
            process::exit(1);
        }
    };

    // XXX we don't need this, reader.bytes() will detect the error when we attempt to read the
    // file and result in error code 21
    if metadata.is_dir() {
        println!("{} is a directory", path.display());
        process::exit(1);
    }

    let reader = BufReader::new(file);
    let count = match Count::count(reader.bytes()) {
        Ok(count) => count,
        Err(e) => {
            println!("error reading file: {}", e);
            process::exit(1);
        }
    };

    // XXX the formatting is such that each field takes up the space of the longest output field
    println!("{} {} {} {} {} {}", count.newlines, count.words, count.chars, count.bytes, count.max_line, path.display());
}
