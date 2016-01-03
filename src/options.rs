extern crate getopts;
use std::env;
use std::slice::Iter;
use std::result;
use std::ffi::OsStr;

#[derive(Debug)]
pub enum Error {
    Usage,
    Version,
    Getopts(getopts::Fail),
}

use std::fmt;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Usage => write!(f, "{}", Options::usage()),
            Error::Version => write!(f, "{}", Options::version()),
            Error::Getopts(ref e) => write!(f, "invalid arguments: {}", e),
        }
    }
}

use std::convert::From;
impl From<getopts::Fail> for Error {
    fn from(e: getopts::Fail) -> Error {
        Error::Getopts(e)
    }
}

pub type Result = result::Result<Options, Error>;

pub struct Options {
    matches: getopts::Matches,
    pub bytes: bool,
    pub chars: bool,
    pub lines: bool,
    pub max_line: bool,
    pub words: bool,
}

impl Options {
    pub fn new() -> Result {
        Self::from_iter(env::args_os())
    }

    pub fn program_name() -> String {
        env::args_os().nth(0)
            .unwrap_or(::std::convert::From::from("rust-wc"))
            .into_string()
            .unwrap_or("rust-wc".to_owned())
    }

    pub fn usage() -> String {
        Self::options().usage(format!(
r"usage: {} [OPTION]... [FILE]...

Print newline, word, and byte counts for each FILE, and a total if more than
one FILE is specified. If no FILEs are provide or if FILE is -, then read from
stdin. A word is a sequence of characters delimited by white space. The
characters count is a count of valid UTF-8 encoded unicode characters.

Which counts are printed can be filtered using the options below. The counts
are always printed in the follwoing order: newline, word, character, byte,
longest line. Counts are separated by whitespace followed by the file name."
, Self::program_name()).as_ref())
    }

    pub fn version() -> String {
        env!("CARGO_PKG_VERSION").to_owned()
    }

    #[cfg(test)]
    pub fn test_args(args: Vec<&str>) -> Result {
        let mut a = vec!["test"];
        a.extend(args.iter());
        Self::from_iter(a.iter())
    }

    #[cfg(test)]
    pub fn test_empty() -> Result {
        Self::from_iter(vec!["test"].iter())
    }

    fn options() -> getopts::Options {
        let mut opts = getopts::Options::new();
        opts.optflag("c", "bytes", "print the byte counts");
        opts.optflag("m", "chars", "print unicode character counts");
        opts.optflag("l", "lines", "print the newline counts");
        opts.optflag("L", "max-line-length", "print the length of the longest line");
        opts.optflag("w", "words", "print the word counts");
        opts.optflag("h", "help", "display this help text and exit");
        opts.optflag("v", "version", "output version information and exit");
        opts
    }

    fn from_iter<I>(args: I) -> Result
        where I: Iterator,
        I::Item: AsRef<OsStr>,
    {
        let options = Self::options();

        // we do skip(1) here because the first argument is the program name
        let matches = try!(options.parse(args.skip(1)));
        if matches.opt_present("h") {
            return Err(Error::Usage);
        }
        if matches.opt_present("v") {
            return Err(Error::Version);
        }

        let mut opts = Options {
            bytes: matches.opt_present("c"),
            chars: matches.opt_present("m"),
            lines: matches.opt_present("l"),
            max_line: matches.opt_present("L"),
            words: matches.opt_present("w"),
            matches: matches,
        };

        // if no options are provided, set some defaults
        if !(opts.bytes || opts.chars || opts.lines || opts.max_line || opts.words) {
            opts.lines = true;
            opts.words = true;
            opts.bytes = true;
        }

        Ok(opts)
    }

    pub fn files(&self) -> Iter<String> {
        self.matches.free.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::Options;

    #[test]
    fn defaults() {
        let args = vec!["test", "file"];
        let opts = Options::from_iter(args.iter()).unwrap();
        assert!(opts.lines);
        assert!(opts.words);
        assert!(opts.bytes);
    }

    #[test]
    fn args_bytes() {
        {
            let args = vec!["test", "-c"];
            let opts = Options::from_iter(args.iter()).unwrap();
            assert!(opts.bytes);
            assert!(!opts.lines);
            assert!(!opts.words);
            assert!(!opts.chars);
            assert!(!opts.max_line);
        }

        {
            let args = vec!["test", "--bytes"];
            let opts = Options::from_iter(args.iter()).unwrap();
            assert!(opts.bytes);
            assert!(!opts.lines);
            assert!(!opts.words);
            assert!(!opts.chars);
            assert!(!opts.max_line);
        }
    }
}
