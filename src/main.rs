use std::process;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::stderr;
use std::fs::File;
use std::error::Error;
extern crate rust_wc;
use rust_wc::counter::Count;
use rust_wc::options;
use rust_wc::options::Options;

fn main() {
    let opts = match Options::new() {
        Ok(opts) => opts,
        Err(e @ options::Error::Usage) => {
            println!("{}", e);
            process::exit(0);
        }
        Err(e @ options::Error::Version) => {
            println!("{}", e);
            process::exit(0);
        }
        Err(e) => {
            writeln!(stderr(), "{}", e).expect("error writing to stderr");
            process::exit(1);
        }
    };

    let mut total = Count::new();
    for file in opts.files() {
        let path = Path::new(file);
        match process_file(path) {
            Err(e) => {
                writeln!(stderr(), "{}: {}", path.display(), e).expect("error writing to stderr");
                println!("{} {}", Count::new().display(&opts), path.display());
            }
            Ok(count) => {
                println!("{} {}", count.display(&opts), path.display());
                total = total + count;
            }
        }
    }

    // print the total count if necessary
    if opts.files().count() > 1 {
        println!("{} total", total.display(&opts));
    }
}

fn process_file(path: &Path) -> Result<Count, Box<Error>> {
    let file = try!(File::open(&path));
    let reader = BufReader::new(file);
    let count = try!(Count::count(reader.bytes()));
    Ok(count)
}
