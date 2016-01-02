extern crate getopts;
use std::env;
use std::slice::Iter;
use std::error::Error;
use std::result;
use std::ffi::OsStr;

pub type Result = result::Result<Options, Box<Error>>;

pub struct Options {
    matches: getopts::Matches,
}

impl Options {
    pub fn new() -> Result {
        Self::from_iter(env::args_os())
    }

    fn from_iter<I>(args: I) -> Result
        where I: Iterator,
        I::Item: AsRef<OsStr>,
    {

        // we do skip(1) here because the first argument is the program name
        let matches = try!(opts.parse(args.skip(1)));

        Ok(Options {
            matches: matches,
        })
    }

    pub fn files(&self) -> Iter<String> {
        self.matches.free.iter()
    }
}
