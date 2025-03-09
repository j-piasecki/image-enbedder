pub struct OffsetIter {
    current: usize,
    index: usize,
    offsets: Vec<u32>,
}

impl OffsetIter {
    pub fn new(offsets: Vec<u32>, skip: u32) -> Self {
        Self {
            current: skip as usize,
            index: 0,
            offsets,
        }
    }
}

impl Iterator for OffsetIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current + self.offsets[self.index] as usize;
        self.current = result + 1;
        self.index = (self.index + 1) % self.offsets.len();
        Some(result)
    }
}
