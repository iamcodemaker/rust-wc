use std::process;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::{stderr, stdin, stdout};
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

    let stdout = stdout();
    let stdout_lock = stdout.lock();
    let mut out = BufWriter::new(stdout_lock);

    let mut total = Count::new();
    for file in opts.files.iter() {
        let result = process_file(file);
        print_count(&mut out, &opts, file, &result);
        if let Ok(count) = result {
            total = total + count;
        }
    }

    match opts.files.len() {
        // no files provided, read from stdin
        0 => print_count(&mut out, &opts, "-", &process_stdin()),
        // print the total count if more than one file was provided
        c if c > 1 => { writeln!(out, "{} total", total.display(&opts)).unwrap(); }
        // else do nothing
        _ => {}
    }
}

fn print_count(out: &mut Write, opts: &Options, file: &str, count_result: &Result<Count, Box<Error>>) {
    match *count_result {
        Err(ref e) => {
            writeln!(stderr(), "{}: {}", file, e).expect("error writing to stderr");
            writeln!(*out, "{} {}", Count::new().display(&opts), file).unwrap();
        }
        Ok(ref count) => {
            writeln!(*out, "{} {}", count.display(&opts), file).unwrap();
        }
    }
}

fn process_file(file: &str) -> Result<Count, Box<Error>> {
    if file == "-" {
        process_stdin()
    }
    else {
        let file = try!(File::open(file));
        let reader = BufReader::new(file);
        Count::count(reader.bytes())
    }
}

fn process_stdin() -> Result<Count, Box<Error>> {
    Count::count(stdin().bytes())
}
