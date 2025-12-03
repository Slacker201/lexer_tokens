
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    start: u64,
    end: u64,
}

impl Span {
    pub fn new(start: u64, end: u64) -> Span {
        Self { start, end }
    }
    pub fn new_usize(start: usize, end: usize) -> Span {
        Self::new(start as u64, end as u64)
    }
}