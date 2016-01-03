use std::fmt;
use counter::Count;
use options::Options;

pub struct Display<'a> {
    count: &'a Count,
    opts: &'a Options,
}

impl<'a> Display<'a> {
    pub fn new(count: &'a Count, opts: &'a Options) -> Self {
        Display {
            count: count,
            opts: opts,
        }
    }
}

impl<'a> fmt::Display for Display<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // XXX the formatting is such that each field takes up the space of the longest output field
        if self.opts.lines { try!(write!(f, " {}", self.count.newlines)); }
        if self.opts.words { try!(write!(f, " {}", self.count.words)); }
        if self.opts.chars { try!(write!(f, " {}", self.count.chars)); }
        if self.opts.bytes { try!(write!(f, " {}", self.count.bytes)); }
        if self.opts.max_line { try!(write!(f, " {}", self.count.max_line)); }
        Ok(())
    }
}
