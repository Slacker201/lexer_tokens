
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    start: u64,
    end: u64,
}

impl Span {
    pub fn new(start: u64, end: u64) -> Span {
        Self { start, end }
    }
}