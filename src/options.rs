extern crate getopts;
use std::env;
use std::slice::Iter;
use std::process;

pub struct Options {
    matches: getopts::Matches,
}

impl Options {
    pub fn new() -> Options {
        let opts = getopts::Options::new();
        let matches = match opts.parse(env::args().skip(1)) {
            Ok(m) => m,
            Err(e) => {
                println!("invalid arguments: {}", e);
                process::exit(1);
            }
        };

        Options {
            matches: matches,
        }
    }

    pub fn files(&self) -> Iter<String> {
        self.matches.free.iter()
    }
}
