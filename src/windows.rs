//! An iterator for subslices of a size.
//!
//! See also: `std::slice::Windows`
//! This structure wasn't right for our purposes because we need the last window even if it's
//! imperfect.

use std::fmt;

#[derive(Debug, Clone)]
struct Windows<'a, T: 'a> {
    elements: &'a [T],
    size: usize,
    step: usize,
}

impl<'a, T> Windows<'a, T> {
    fn new(elements: &'a [T], size: usize, step: usize) -> Self {
        assert!(size > 0);
        assert!(step > 0);
        assert!(step <= size);
        Windows { elements: elements, size: size, step: step }
    }
}

impl<'a, T> Iterator for Windows<'a, T>
        where T: fmt::Debug {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.elements.len() == 0 {
            return None
        }

        let size = if self.size > self.elements.len() {
            self.elements.len()
        } else {
            self.size
        };

        let ret = &self.elements[..size];

        if self.size == size {
            self.elements = &self.elements[self.step..];
        } else {
            self.elements = &[];
        }
        Some(ret)
    }
}


trait Windower {
    pub fn into_windows(&self, size: usize, step: usize) -> Windows {
    }
}


#[cfg(test)]
mod test {
    use super::Windows;

    #[test]
    fn can_use_next() {
        let elems = &[1, 2, 3, 4];
        let mut win = Windows::new(elems, 2, 1);
        assert_eq!(win.next().unwrap(), &[1, 2]);
        assert_eq!(win.next().unwrap(), &[2, 3]);
        assert_eq!(win.next().unwrap(), &[3, 4]);
        assert!(win.next().is_none());
    }

    #[test]
    fn handles_incomplete_windows() {
        let elems = &[1, 2, 3, 4];
        let mut win = Windows::new(elems, 3, 2);
        assert_eq!(win.next().unwrap(), &[1, 2, 3]);
        assert_eq!(win.next().unwrap(), &[3, 4]);
        assert!(win.next().is_none());
    }

    #[test]
    fn handles_long_step() {
        let elems = &[1, 2, 3, 4];
        let mut win = Windows::new(elems, 3, 6);
        assert_eq!(win.next().unwrap(), &[1, 2, 3]);
        assert!(win.next().is_none());
    }
}
