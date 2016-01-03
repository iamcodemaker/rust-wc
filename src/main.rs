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
    let opts = match Options::new() {
        Ok(opts) => opts,
        Err(e) => {
            println!("invalid arguments: {}", e);
            process::exit(1);
        }
    };

    if opts.files().count() != 1 {
        println!("error: invalid arguments");
        process::exit(1);
    }

    let path = opts.files().nth(0).unwrap();
    let path = Path::new(path);
    let count = match process_file(path) {
        Ok(count) => count,
        Err(e) => {
            println!("error processing file: {}", e);
            process::exit(1);
        }
    };

    println!("{} {}", count.display(&opts), path.display());
}

fn process_file(path: &Path) -> Result<Count, Box<Error>> {
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
    Ok(count)
}
