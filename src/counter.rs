pub struct Count {
    pub newlines: u32,
    pub words: u32,
    pub bytes: u32,
    pub chars: u32,
    pub max_line_length: u32,
}

impl Count {
    pub fn new() -> Self {
        Count {
            newlines: 0,
            words: 0,
            bytes: 0,
            chars: 0,
            max_line_length: 0,
        }
    }
}

