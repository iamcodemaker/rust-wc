extern crate getopts;
use std::env;
use std::slice::Iter;
use std::error::Error;
use std::result;

pub type Result = result::Result<Options, Box<Error>>;

pub struct Options {
    matches: getopts::Matches,
}

impl Options {
    pub fn new() -> Result {
        let opts = getopts::Options::new();

        // we do skip(1) here because the first argument is the program name
        let matches = try!(opts.parse(env::args().skip(1)));

        Ok(Options {
            matches: matches,
        })
    }

    pub fn files(&self) -> Iter<String> {
        self.matches.free.iter()
    }
}
