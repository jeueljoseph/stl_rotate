use std::ops::{Bound, RangeBounds};
pub trait Rotate<T> {
    fn rotate<R: RangeBounds<usize>>(&mut self, range: R, new_first: usize);
}

impl<T> Rotate<T> for Vec<T> {
    fn rotate<R: RangeBounds<usize>>(&mut self, range: R, new_first: usize) {
        let first = match range.start_bound() {
            Bound::Excluded(i) => i + 1,
            Bound::Included(i) => *i,
            Bound::Unbounded => 0,
        };

        let last = match range.end_bound() {
            Bound::Excluded(i) => i - 1,
            Bound::Included(i) => *i,
            Bound::Unbounded => self.len() - 1,
        };

        let mut read = new_first;
        let mut write = first;
        let mut next_read = first;

        while write < last + 1 {
            while read != last + 1 {
                if write == next_read {
                    next_read = read
                }
                self.swap(write, read);
                write = write + 1;
                read = read + 1;
            }

            read = next_read;
            next_read = write;
        }
    }
}
