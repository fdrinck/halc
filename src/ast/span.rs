use std::ops::Range;

#[derive(Debug, Copy, Clone)]
pub struct Span(u64);

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start >> 48 == 0);
        assert!(start <= end);
        let length = end - start;
        assert!(length >> 16 == 0);

        // match length to token size
        Self(((start as u64) << 16) | (length & 0xFFFF) as u64)
    }

    pub fn start(&self) -> usize {
        (self.0 >> 16) as usize
    }

    pub fn length(&self) -> usize {
        (self.0 & 0xFFFF) as usize
    }

    pub fn end(&self) -> usize {
        self.length() + self.start()
    }

    pub fn slice<'s>(&self, source: &'s str) -> &'s str {
        &source[self.start()..self.end()]
    }

    pub fn range(&self) -> Range<usize> {
        self.start()..self.end()
    }
}
