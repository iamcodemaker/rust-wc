extern crate getopts;
use std::env;
use std::result;
use std::ffi::OsStr;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::string;

#[derive(Debug)]
pub enum Error {
    Usage,
    Version,
    Files0FromWithFiles,
    Getopts(getopts::Fail),
    Io(io::Error),
    Utf8(string::FromUtf8Error),
}

use std::fmt;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Usage => write!(f, "{}", Options::usage()),
            Error::Version => write!(f, "{}", Options::version()),
            Error::Files0FromWithFiles => write!(f, "invalid arguments: can't use --files0-from with a FILEs list"),
            Error::Getopts(ref e) => write!(f, "invalid arguments: {}", e),
            Error::Io(ref e) => write!(f, "error reading file list: {}", e),
            Error::Utf8(ref e) => write!(f, "error reading file list, invalid utf8: {}", e),
        }
    }
}

use std::convert::From;
impl From<getopts::Fail> for Error {
    fn from(e: getopts::Fail) -> Error {
        Error::Getopts(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(e: string::FromUtf8Error) -> Error {
        Error::Utf8(e)
    }
}

pub type Result = result::Result<Options, Error>;

pub struct Options {
    pub files: Vec<String>,
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
r"Usage:
  {0} [OPTION]... [FILE]...
  {0} [OPTION]... --files0-from F

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
        format!("{} v{}", Self::program_name(), env!("CARGO_PKG_VERSION"))
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
        opts.optopt("", "files0-from", "read input file list from the specified file containing a NUL-terminated list of file names; use - to read from stdin", "F");
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

        let files0_from = match  matches.opt_str("files0-from") {
            Some(files0_from) => {
                if !matches.free.is_empty() {
                    // using --files0-from with FILEs is not allowed
                    return Err(Error::Files0FromWithFiles);
                }
                Some(try!(load_files_from(&files0_from)))
            }
            None => None,
        };

        let mut opts = Options {
            bytes: matches.opt_present("c"),
            chars: matches.opt_present("m"),
            lines: matches.opt_present("l"),
            max_line: matches.opt_present("L"),
            words: matches.opt_present("w"),
            files: files0_from.unwrap_or(matches.free),
        };

        // if no options are provided, set some defaults
        if !(opts.bytes || opts.chars || opts.lines || opts.max_line || opts.words) {
            opts.lines = true;
            opts.words = true;
            opts.bytes = true;
        }

        Ok(opts)
    }

    /// Return `true` if only the bytes option is set.
    ///
    /// If bytes is the only option, additional optimizations can be done.
    pub fn only_bytes(&self) -> bool {
        self.bytes && !(self.chars || self.lines || self.max_line || self.words)
    }
}

fn load_files_from(file: &str) -> result::Result<Vec<String>, Error> {
    if file == "-" {
        load_files_from_stdin()
    }
    else {
        let file = try!(File::open(file));
        let reader = io::BufReader::new(file);
        load_files_from_iter(reader.bytes())
    }
}

fn load_files_from_stdin() -> result::Result<Vec<String>, Error> {
    load_files_from_iter(io::stdin().bytes())
}

fn load_files_from_iter<I>(bytes: I) -> result::Result<Vec<String>, Error>
   where I: Iterator<Item=io::Result<u8>>
{
    let mut vec = Vec::new();
    let mut vec_string = Vec::new();
    for b in bytes {
        match try!(b) {
            0 => {
                vec.push(try!(String::from_utf8(vec_string.clone())));
                vec_string.clear();
            }
            b => vec_string.push(b),
        }
    }

    // add the final string, in case it wasn't null terminated
    if vec.is_empty() || !vec_string.is_empty() {
        vec.push(try!(String::from_utf8(vec_string)));
    }

    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::Options;
    use super::Error;
    use super::load_files_from_iter;
    use std::io;

    #[test]
    fn defaults() {
        let args = vec!["test", "file"];
        let opts = Options::from_iter(args.iter()).unwrap();
        assert!(opts.lines);
        assert!(opts.words);
        assert!(opts.bytes);
        assert_eq!(opts.files[0], "file");
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
            assert!(opts.only_bytes());
        }

        {
            let args = vec!["test", "--bytes"];
            let opts = Options::from_iter(args.iter()).unwrap();
            assert!(opts.bytes);
            assert!(!opts.lines);
            assert!(!opts.words);
            assert!(!opts.chars);
            assert!(!opts.max_line);
            assert!(opts.only_bytes());
        }
    }

    #[test]
    fn files0_from_with_files() {
        let args = vec!["test", "--files0-from", "file", "other-file"];
        match Options::from_iter(args.iter()) {
            Err(Error::Files0FromWithFiles) => {} // do nothing, this error is expected
            Ok(_) => panic!("did not expect this to succeed"),
            Err(e) => panic!("did not expect error {}", e),
        }
    }

    fn vec_from_string(s: &str) -> Vec<io::Result<u8>> {
        s.bytes()
            .map(|c| Ok(c))
            .collect()
    }

    #[test]
    fn files0_from_no_null_term() {
        let vec = load_files_from_iter(vec_from_string("a\0b\0c").into_iter()).unwrap();
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], "a");
        assert_eq!(vec[1], "b");
        assert_eq!(vec[2], "c");
    }

    #[test]
    fn files0_from_one_item() {
        let vec = load_files_from_iter(vec_from_string("one").into_iter()).unwrap();
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], "one");
    }

    #[test]
    fn files0_from_null_term() {
        let vec = load_files_from_iter(vec_from_string("a\0b\0c\0").into_iter()).unwrap();
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], "a");
        assert_eq!(vec[1], "b");
        assert_eq!(vec[2], "c");
    }

    #[test]
    fn files0_from_empty() {
        let vec = load_files_from_iter(vec_from_string("").into_iter()).unwrap();
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], "");
    }
}
