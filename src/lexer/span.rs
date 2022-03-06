use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Loc {
    pub byte_offset: usize,
    pub line: usize,
    pub col: usize,
}

impl Default for Loc {
    fn default() -> Self {
        Self {
            byte_offset: 0,
            line: 1,
            col: 1,
        }
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line: {} column: {}", self.line, self.col)
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Span {
    pub start: Loc,
    pub end: Loc,
}
