use bio::io::fasta;
use std::hash::Hash;

use countrs::Counter;
use windowrs::Windows;

use crate::bedgraph::BGBlock;

/// Calculate the GC% of a sequence.
///
/// Examples:
///
/// ```rust
/// use riprap::stats;
///
/// let result = stats::base_content(Window::new(1, 10, b"ATGC"), b"GC");
/// assert_eq!(Window::new(1, 10, 0.5), result);
///
/// let result = stats::base_content(Window::new(1, 10, b"ATGC"), b"TGC");
/// assert_eq!(Window::new(1, 10, 0.75), result);
/// ```
pub fn base_content<T: Eq + Hash + Clone>(seq: &[T], bases: &[T]) -> f64 {
    let counter: Counter<T> = seq.iter().cloned().collect();
    counter.prop_sum(bases)
}

/// Calculate the composite RIP index (CRI) of a sequence.
///
/// Examples:
///
/// ```rust
/// use riprap::stats;
///
/// let result = stats::cri(b"TACATGT");
/// assert_eq!(result, 0.0);
/// ```
pub fn cri(seq: &[u8]) -> f64 {
    let freq: Counter<[u8; 2]> = seq.windows(2).map(|x| [x[0], x[1]]).collect();

    let ta = freq.count(b"TA") as f64;
    let at = freq.count(b"AT") as f64;

    let ca = freq.count(b"CA") as f64;
    let ac = freq.count(b"AC") as f64;

    let gt = freq.count(b"GT") as f64;
    let tg = freq.count(b"TG") as f64;

    let num = ca + tg;
    let denom = ac + gt;
    let ratio = if denom == 0.0 { 0.0 } else { num / denom };

    let offset = ta / at;
    offset - ratio
}

/// Apply a function along a sequence in windows.
///
/// Examples:
///
/// ```rust
/// extern crate bio;
/// extern crate riprap;
///
/// use bio::io::fasta;
/// use riprap::stats;
/// use riprap::bedgraph::BGBlock;
///
/// let rec = fasta::Record::with_attrs(
///     "test_id",
///     None,
///     b"ATGC"
/// );
///
/// let result = stats::sliding_windows(&rec, 2, 1, |_| 1.0);
/// assert_eq!(result, vec![
///     BGBlock::new("test_id", 0, 2, 1.0),
///     BGBlock::new("test_id", 1, 3, 1.0),
///     BGBlock::new("test_id", 2, 4, 1.0)
/// ]);
/// ```
pub fn sliding_windows<'a, F>(
    record: &fasta::Record,
    size: usize,
    step: usize,
    f: F,
) -> Vec<BGBlock>
where
    F: Fn(&[u8]) -> f64,
{
    let seq = record.seq();
    let id = record.id();

    Windows::new(seq, size, step)
        .map(|win| BGBlock::new(id, win.start, win.end, f(win.value)))
        .collect()
}
