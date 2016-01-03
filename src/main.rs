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

    for file in opts.files() {
        let path = Path::new(file);
        match process_file(path) {
            Err(e) => {
                println!("{}: {}", path.display(), e);
                println!("{} {}", Count::new().display(&opts), path.display());
            }
            Ok(count) => {
                println!("{} {}", count.display(&opts), path.display());
            }
        }
    }
}

fn process_file(path: &Path) -> Result<Count, Box<Error>> {
    let file = try!(File::open(&path));
    let reader = BufReader::new(file);
    let count = try!(Count::count(reader.bytes()));
    Ok(count)
}
