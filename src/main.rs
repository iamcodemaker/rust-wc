use std::process;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
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
    match process_file(Path::new(path)) {
        Ok(_) => {}
        Err(e) => {
            println!("error processing file: {}", e);
            process::exit(1);
        }
    }
}

fn process_file(path: &Path) -> Result<(), Box<Error>> {
    let file = try!(File::open(&path));
    let metadata = try!(file.metadata());

    // XXX we don't need this, reader.bytes() will detect the error when we attempt to read the
    // file and result in error code 21
    if metadata.is_dir() {
        println!("{} is a directory", path.display());
        process::exit(1);
    }

    let reader = BufReader::new(file);
    let count = try!(Count::count(reader.bytes()));

    // XXX the formatting is such that each field takes up the space of the longest output field
    println!("{} {} {} {} {} {}", count.newlines, count.words, count.chars, count.bytes, count.max_line, path.display());
    Ok(())
}
