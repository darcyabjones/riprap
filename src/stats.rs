use bio::io::fasta;
use std::hash::Hash;
use windowrs::Windows;

use countrs::Counter;

/// Calculate the GC% of a sequence.
///
/// Examples:
///
/// ```rust
/// use riprap::stats;
/// let result = stats::base_content(b"ATGC", b"GC");
/// assert_eq!(result, 0.5);
///
/// let result = stats::base_content(b"ATGC", b"TGC");
/// assert_eq!(result, 0.75);
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
/// The implementation isn't perfect.
/// It won't handle cases where the last window isn't window size.
/// It will omit the last window in that case.
/// Probably calls for a custom trait/object.
///
/// Examples:
///
/// ```rust
/// extern crate bio;
/// extern crate riprap;
///
/// use riprap::stats;
/// use bio::io::fasta::Record;
///
/// let rec = Record::with_attrs(
///     "test_id",
///     None,
///     b"ATGC"
/// );
///
/// let result = stats::sliding_windows(&rec, 2, 1, |_| 1.0);
/// assert_eq!(result, vec![
///     ("test_id", 0, 2, 1.0),
///     ("test_id", 1, 3, 1.0),
///     ("test_id", 2, 4, 1.0)
/// ]);
/// ```
pub fn sliding_windows<F: Fn(&[u8]) -> f64>(
    record: &fasta::Record,
    size: usize,
    step: usize,
    f: F,
) -> Vec<(&str, usize, usize, f64)> {
    let seq = record.seq();

    let wins = Windows::new(seq, size, step).map(f);
    let tups = (0..)
        .step_by(step)
        .zip(wins)
        .map(|(i, score)| (record.id(), i, i + size, score))
        .collect();

    tups
}

