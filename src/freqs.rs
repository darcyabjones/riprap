//! A simple convenience datastructure for counting occurrences of elements.
//!
//! Most of this is copied from <https://github.com/BurntSushi/rust-stats>
//! I wanted to be able to initialise the hashmap with different capacities,
//! and to add some more documentation etc.


use std::collections::hash_map::{HashMap, Entry};
use std::fmt;
use std::hash::Hash;
use std::iter::{FromIterator, IntoIterator};
use std::default::Default;

/// A data structure for exact frequency counts.
#[derive(Clone)]
pub struct Frequencies<T> {
    data: HashMap<T, u64>,
}

impl<T: fmt::Debug + Eq + Hash> fmt::Debug for Frequencies<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T: Eq + Hash> Frequencies<T> {
    /// Create a new frequency table with no samples and capacity `cap`.
    ///
    /// # Examples
    ///
    /// ```
    /// use 
    pub fn new(cap: usize) -> Self {
        Frequencies { data: HashMap::with_capacity(cap) }
    }

    /// Add a sample to the frequency table.
    pub fn add(&mut self, v: T) {
        match self.data.entry(v) {
            Entry::Vacant(count) => { count.insert(1); },
            Entry::Occupied(mut count) => { *count.get_mut() += 1; },
        }
    }

    /// Return the number of occurrences of `v` in the data.
    pub fn count(&self, v: &T) -> u64 {
        self.data.get(v).map(|&v| v).unwrap_or(0)
    }

    /// Returns the cardinality of the data.
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T: Eq + Hash> Default for Frequencies<T> {
    fn default() -> Self {
        Self::new(100000)
    }
}

impl<T: Eq + Hash> FromIterator<T> for Frequencies<T> {
    fn from_iter<I: IntoIterator<Item=T>>(it: I) -> Frequencies<T> {
        let mut v = Frequencies::default();
        v.extend(it);
        v
    }
}

impl<T: Eq + Hash> Extend<T> for Frequencies<T> {
    fn extend<I: IntoIterator<Item=T>>(&mut self, it: I) {
        for sample in it {
            self.add(sample);
        }
    }
}



#[cfg(test)]
mod test {
    use super::Frequencies;

    #[test]
    fn ranked() {
        let mut counts = Frequencies::new(10);
        counts.extend(vec![1usize, 1, 2, 2, 2, 2, 2, 3, 4, 4, 4].into_iter());
        assert_eq!(counts.most_frequent()[0], (&2, 5));
        assert_eq!(counts.least_frequent()[0], (&3, 1));
    }
}
