use std::hash::Hash;

use countrs::Counter;


fn div_safe(num: f64, denom: f64) -> f64 {
    if denom == 0.0 { 0.0 } else { num / denom }
}


/// Count the frequencies of dinucleotides in a byte string.
///
/// Examples:
///
/// ```rust
/// use riprap::stats;
/// let result = stats::dinucleotide_counts(b"TACATGTN");
/// assert_eq!(result.count(b"TA"), 1);
/// ```
pub fn dinucleotide_counts(seq: &[u8]) -> Counter<[u8; 2]> {
    seq.windows(2).map(|x| [x[0], x[1]]).collect()
}


/// Count the frequencies of trinucleotides in a byte string.
///
/// Examples:
///
/// ```rust
/// use riprap::stats;
/// let result = stats::trinucleotide_counts(b"TACATGTN");
/// assert_eq!(result.count(b"TAC"), 1);
/// ```
pub fn trinucleotide_counts(seq: &[u8]) -> Counter<[u8; 3]> {
    seq.windows(3).map(|x| [x[0], x[1], x[3]]).collect()
}


/// Calculate the ratio of ta to at dinucleotides.
pub fn margolin1(counts: &Counter<[u8; 2]>) -> f64 {
    let ta = counts.count(b"TA") as f64;
    let at = counts.count(b"AT") as f64;

    ta / at
}


pub fn margolin2(counts: &Counter<[u8; 2]>) -> f64 {
    let ca = counts.count(b"CA") as f64;
    let ac = counts.count(b"AC") as f64;

    let gt = counts.count(b"GT") as f64;
    let tg = counts.count(b"TG") as f64;

    div_safe(ca + tg, ac + gt)
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
pub fn cri(counts: &Counter<[u8; 2]>) -> f64 {
    margolin1(&counts) - margolin2(&counts)
}


pub fn count<T: Eq + Hash>(counts: &Counter<T>, element: &T) -> f64 {
    counts.count(element) as f64
}

pub fn count_rc<T: Eq + Hash>(counts: &Counter<T>, elem: &T) -> f64 {


}
