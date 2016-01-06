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

    fn field_width(&self) -> usize {
        use std::cmp::min;
        use std::cmp::max;

        fn digit_count(num: u64) -> usize {
            if num > 0 { ((num as f64).log10() + 1f64) as usize }
            else { 1 }
        }

        // for a single file scale the width
        if self.opts.files.len() <= 1 {
            // calculate the width of the longest field and scale the field with based on that. Use a
            // max field size of 7
            let mut digits = 0;
            if self.opts.lines { digits = max(digits, digit_count(self.count.newlines)); }
            if self.opts.words { digits = max(digits, digit_count(self.count.words)); }
            if self.opts.chars { digits = max(digits, digit_count(self.count.chars)); }
            if self.opts.bytes { digits = max(digits, digit_count(self.count.bytes)); }
            if self.opts.max_line { digits = max(digits, digit_count(self.count.max_line)); }
            min(digits, 7)
        }
        // for multiple files width is always 7
        else {
            7
        }
    }
}

impl<'a> fmt::Display for Display<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = self.field_width();
        let mut padding = "";
        if self.opts.lines { try!(write!(f, "{}{: >width$}", padding, self.count.newlines, width = width)); padding = " "; }
        if self.opts.words { try!(write!(f, "{}{: >width$}", padding, self.count.words, width = width)); padding = " "; }
        if self.opts.chars { try!(write!(f, "{}{: >width$}", padding, self.count.chars, width = width)); padding = " "; }
        if self.opts.bytes { try!(write!(f, "{}{: >width$}", padding, self.count.bytes, width = width)); padding = " "; }
        if self.opts.max_line { try!(write!(f, "{}{: >width$}", padding, self.count.max_line, width = width)); }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use options::Options;
    use counter::Count;
    use std::fmt::Write;

    #[test]
    fn everything() {
        let mut s = String::new();
        let mut count = Count::new();
        count.newlines = 0;
        count.words = 1;
        count.chars = 2;
        count.bytes = 3;
        count.max_line = 4;

        let mut opts = Options::test_empty().unwrap();
        opts.lines = true;
        opts.words = true;
        opts.chars = true;
        opts.bytes = true;
        opts.max_line = true;
        write!(s, "{}", count.display(&opts)).unwrap();
        assert_eq!(s, "0 1 2 3 4");
    }

    #[test]
    fn everything_fields() {
        let mut s = String::new();
        let mut count = Count::new();
        count.newlines = 0;
        count.words = 10;
        count.chars = 2;
        count.bytes = 3;
        count.max_line = 4;

        let mut opts = Options::test_empty().unwrap();
        opts.lines = true;
        opts.words = true;
        opts.chars = true;
        opts.bytes = true;
        opts.max_line = true;
        write!(s, "{}", count.display(&opts)).unwrap();
        assert_eq!(s, " 0 10  2  3  4");
    }

    #[test]
    fn everything_fields_long() {
        let mut s = String::new();
        let mut count = Count::new();
        count.newlines = 0;
        count.words = 1;
        count.chars = 2_000_000_000;
        count.bytes = 3;
        count.max_line = 4;

        let mut opts = Options::test_empty().unwrap();
        opts.lines = true;
        opts.words = true;
        opts.chars = true;
        opts.bytes = true;
        opts.max_line = true;
        write!(s, "{}", count.display(&opts)).unwrap();
        assert_eq!(s, "      0       1 2000000000       3       4");
    }

    /// With multiple files, field with is 7 instead of variable.
    #[test]
    fn multiple_files() {
        let mut s = String::new();
        let mut count = Count::new();
        count.newlines = 0;
        count.words = 1;
        count.chars = 2;
        count.bytes = 3;
        count.max_line = 4;

        let args = vec!["file", "file"];
        let mut opts = Options::test_args(args).unwrap();
        opts.lines = true;
        opts.words = true;
        opts.chars = true;
        opts.bytes = true;
        opts.max_line = true;
        write!(s, "{}", count.display(&opts)).unwrap();
        assert_eq!(s, "      0       1       2       3       4");
    }
}
