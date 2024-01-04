pub mod io;
pub mod errors;

#[derive(Debug)]
pub struct SizedPointer {
    index: usize,
    size: usize
}

impl SizedPointer {
    pub fn new(index: usize, size: usize) -> Self {
        Self {
            index, size
        }
    }
    
    pub fn as_range(&self) -> std::ops::Range<usize> {
        return self.index..(self.index + self.size)
    }
}
