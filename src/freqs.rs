//! A simple convenience datastructure for counting occurrences of elements.
//!
//! Most of this is copied from <https://github.com/BurntSushi/rust-stats>
//! I wanted to be able to initialise the hashmap with different capacities,
//! and to add some more documentation etc.


use std::collections::hash_map::{HashMap, Entry, Keys, Values};
use std::fmt;
use std::hash::Hash;
use std::iter::{FromIterator, IntoIterator};
use std::default::Default;

/// A data structure for exact frequency counts.
#[derive(Clone)]
pub struct Counter<T> {
    data: HashMap<T, u64>,
}

impl<T: fmt::Debug + Eq + Hash> fmt::Debug for Counter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T: Eq + Hash> Counter<T> {
    /// Create a new frequency table with no samples and capacity `cap`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use riprap::freqs::Counter;
    ///
    /// let mut counter = Counter::new(20);
    ///
    /// // Now use counter object
    /// counter.extend("aabcde".chars());
    /// assert_eq!(counter.count(&'a'), 2);
    /// ```
    pub fn new(cap: usize) -> Self {
        Counter { data: HashMap::with_capacity(cap) }
    }

    /// Add a sample to the frequency table.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use riprap::freqs::Counter;
    ///
    /// let mut counter = Counter::new(5);
    ///
    /// counter.add('a');
    /// ```
    pub fn add(&mut self, v: T) {
        match self.data.entry(v) {
            Entry::Vacant(count) => { count.insert(1); },
            Entry::Occupied(mut count) => { *count.get_mut() += 1; },
        }
    }

    /// Returns the number of unique samples in the data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use riprap::freqs::Counter;
    ///
    /// let mut counter = Counter::new(5);
    ///
    /// counter.add('a');
    /// counter.add('a');
    /// counter.add('b');
    /// assert_eq!(counter.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the total number of samples in the data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use riprap::freqs::Counter;
    ///
    /// let mut counter = Counter::new(5);
    ///
    /// counter.add('a');
    /// counter.add('a');
    /// counter.add('b');
    /// assert_eq!(counter.size(), 3);
    /// ```
    pub fn size(&self) -> usize {
        self.data.values().fold(0, |acc, x| acc + x) as usize
    }

    /// Return the number of occurrences of `v` in the data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use riprap::freqs::Counter;
    ///
    /// let mut counter = Counter::new(5);
    ///
    /// assert_eq!(counter.count(&'a'), 0);
    /// counter.add('a');
    /// assert_eq!(counter.count(&'a'), 1);
    /// ```
    pub fn count(&self, element: &T) -> u64 {
        *self.data.get(element).unwrap_or(&0)
    }

    /// Return the sum of occurrences of a list of elements in the data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use riprap::freqs::Counter;
    /// let counter: Counter<u8> = "atgc".bytes().collect();
    ///
    /// let count = counter.count_sum(b"gc");
    /// assert_eq!(count, 2);
    /// ```
    pub fn count_sum(&self, elements: &[T]) -> u64 {
        elements.iter()
            .map(|e| self.count(e))
            .fold(0, |acc, x| acc + x)
    }


    /// Return the proportion of occurrences of `v` in the data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use riprap::freqs::Counter;
    ///
    /// let mut counter = Counter::new(5);
    ///
    /// assert_eq!(counter.prop(&'a'), 0.0);
    /// counter.add('a');
    /// assert_eq!(counter.prop(&'a'), 1.0);
    /// ```
    pub fn prop(&self, element: &T) -> f64 {
        let size = self.size();

        if size == 0 {
            return 0.0
        }

        let count = self.count(element);
        (count as f64) / (size as f64)
    }

    /// Return the sum of proportions of a list of elements in the data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use riprap::freqs::Counter;
    /// let counter: Counter<u8> = "atgc".bytes().collect();
    ///
    /// let prop = counter.prop_sum(b"gc");
    /// assert_eq!(prop, 0.5);
    /// ```
    pub fn prop_sum(&self, elements: &[T]) -> f64 {
        let size = self.size();

        if size == 0 {
            return 0.0
        }

        let counts = self.count_sum(elements);
        (counts as f64) / (size as f64)
    }

}

impl<T: Eq + Hash> Default for Counter<T> {

    /// Initialises the Counter counter with predefined number.
    ///
    /// Note that the capacity will resize dynamically if you exceed this
    /// default.
    fn default() -> Self {
        Self::new(100)
    }
}

impl<T: Eq + Hash> FromIterator<T> for Counter<T> {

    /// Creates a new Counter from an iterable.
    ///
    /// Rather than initialise and extend the counter as two
    /// steps, you can perform it in one.
    /// This enables some syntactic sugar via the `.collect()` method and 
    /// the object is not mutable.
    /// 
    /// Note. To use the `::from_iter` form, you must explicitly use the
    /// FromIterator trait. The `collect()` version does not need this.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use riprap::freqs::Counter;
    /// use std::iter::FromIterator;
    /// 
    /// let c1 = Counter::from_iter("aaabbbc".chars());
    /// assert_eq!(c1.count(&'b'), 3);
    ///
    /// let c2: Counter<char> = "aaabbbc".chars().collect();
    /// assert_eq!(c2.count(&'b'), 3);
    /// ```
    fn from_iter<I: IntoIterator<Item=T>>(it: I) -> Self {
        let mut v = Counter::default();
        v.extend(it);
        v
    }
}

impl<T: Eq + Hash> Extend<T> for Counter<T> {

    /// Adds each elements from an iterator to the table.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use riprap::freqs::Counter;
    /// 
    /// let mut counter = Counter::default();
    /// counter.extend("aaaabbbc".chars());
    ///
    /// assert_eq!(counter.count(&'a'), 4);
    /// ```
    fn extend<I: IntoIterator<Item=T>>(&mut self, it: I) {
        for sample in it {
            self.add(sample);
        }
    }
}


#[cfg(test)]
mod test {
    use super::Counter;
    use std::iter::FromIterator;

    #[test]
    fn can_add_entries() {
        let mut counts = Counter::new(10);
        assert_eq!(counts.len(), 0);

        counts.add(b'a');
        assert_eq!(counts.count(&b'a'), 1);
        assert_eq!(counts.count(&b'b'), 0);
        assert_eq!(counts.len(), 1);

        counts.add(b'a');
        counts.add(b'b');
        assert_eq!(counts.count(&b'a'), 2);
        assert_eq!(counts.count(&b'b'), 1);
        assert_eq!(counts.len(), 2);
    }


    #[test]
    fn can_extend_entries() {
        let mut counts = Counter::new(10);
        assert_eq!(counts.len(), 0);

        counts.extend(vec!['a', 'a', 'b']);
        assert_eq!(counts.count(&'a'), 2);
        assert_eq!(counts.count(&'b'), 1);
        assert_eq!(counts.count(&'c'), 0);
        assert_eq!(counts.len(), 2);
    }


    #[test]
    fn can_expand_capacity() {
        let mut counts = Counter::new(5);
        assert_eq!(counts.len(), 0);

        counts.extend("aaabbcdefghijkl".chars());
        assert_eq!(counts.count(&'a'), 3);
        assert_eq!(counts.count(&'b'), 2);
        assert_eq!(counts.count(&'c'), 1);
        assert_eq!(counts.count(&'d'), 1);
        assert_eq!(counts.len(), 12);
    }


    #[test]
    fn can_create_from_iter() {
        let counts = Counter::from_iter(vec!['a', 'b', 'b']);
        assert_eq!(counts.len(), 2);

        assert_eq!(counts.count(&'a'), 1);
        assert_eq!(counts.count(&'b'), 2);
        assert_eq!(counts.count(&'c'), 0);

        let counts: Counter<char> = "abb".chars().collect();
        assert_eq!(counts.len(), 2);

        assert_eq!(counts.count(&'a'), 1);
        assert_eq!(counts.count(&'b'), 2);
        assert_eq!(counts.count(&'c'), 0);
    }
}
