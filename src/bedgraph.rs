//! Simple bedgraph struct and writing utility.

#[derive(Debug, PartialEq)]
pub struct BGBlock {
    pub seqid: String,
    pub start: usize,
    pub end: usize,
    pub score: f64,
}

impl BGBlock {
    pub fn new(seqid: &str, start: usize, end: usize, score: f64) -> Self {
        BGBlock {
            seqid: seqid.to_string(),
            start: start,
            end: end,
            score: score,
        }
    }
}

impl std::fmt::Display for BGBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}",
            self.seqid, self.start, self.end, self.score
        )
    }
}
