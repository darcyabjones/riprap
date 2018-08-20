use bio::io::fasta;

use freqs::Frequencies;

/// Calculate the GC% of a sequence.
///
/// Examples:
///
/// ```rust
/// use riprap::stats;
/// let result = stats::gc_content(b"ATGC");
/// assert_eq!(result, 0.5);
/// ```
pub fn gc_content(seq: &[u8]) -> f64 {

    let mut freq = Frequencies::new(6);
    freq.extend(seq);

    let gc = (freq.count(&&b'G') + freq.count(&&b'C')) as f64;
    let len = seq.len() as f64;

    gc / len
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
    let dinucs = seq.windows(2).map(|x| [x[0], x[1]]);

    let mut di_freq = Frequencies::new(25);
    di_freq.extend(dinucs);

    let ta = di_freq.count(b"TA") as f64;
    let at = di_freq.count(b"AT") as f64;

    let ca = di_freq.count(b"CA") as f64;
    let ac = di_freq.count(b"AC") as f64;

    let gt = di_freq.count(b"GT") as f64;
    let tg = di_freq.count(b"TG") as f64;

    let num = ca + tg;
    let denom = ac + gt;
    let ratio = if denom == 0.0 {
        0.0
    } else {
        num / denom
    };

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
/// use riprap::stats;
///
/// let result = stats::sliding_windows(&[1, 2, 3, 4], 2, 1, |x| 1.0);
/// assert_eq!(result, vec![1.0, 1.0, 1.0]);
pub fn sliding_windows<F: Fn(&[u8]) -> f64>(seq: &[u8], size: usize, step: usize, f: F) -> Vec<f64> {
    seq.windows(size).step_by(step).map(f).collect()
}


pub fn run_sliding_windows<F: Fn(&[u8]) -> f64>(record: &fasta::Record, size: usize, step: usize, f: F) -> Vec<(&str, usize, usize, f64)> {
    let frac = sliding_windows(&record.seq(), size, step, f);

    let mut output = Vec::new();
    for (i, j) in (0..).step_by(step).zip(frac) {
        output.push((record.id(), i, i + size, j));
    }
    output
} 
