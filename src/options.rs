extern crate getopts;
use std::env;
use std::slice::Iter;
use std::error::Error;
use std::result;
use std::ffi::OsStr;

pub type Result = result::Result<Options, Box<Error>>;

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
        opts.optflag("m", "chars", "print the character counts");
        opts.optflag("l", "lines", "print the newline counts");
        opts.optflag("L", "max-line-length", "print the length of the longest line");
        opts.optflag("w", "words", "print the word counts");
        opts
    }

    fn from_iter<I>(args: I) -> Result
        where I: Iterator,
        I::Item: AsRef<OsStr>,
    {
        let opts = Self::options();

        // we do skip(1) here because the first argument is the program name
        let matches = try!(opts.parse(args.skip(1)));

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
