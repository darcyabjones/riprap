extern crate bio;

pub mod freqs;

pub mod gc {
    use bio::io::fasta;
    use freqs::Frequencies;

    pub fn gc_content(seq: &[u8]) -> f64 {

        let mut freq = Frequencies::new(6);
        freq.extend(seq);

        let gc = (freq.count(&&b'G') + freq.count(&&b'C')) as f64;
        let len = seq.seq().len() as f64;

        gc / len
    }
}

pub mod cri {
    use freqs::Frequencies;

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
}

pub mod runner {

    use std::path::Path;
    use bio::io::fasta;
    use cri;

    pub fn run(path: &Path) {

        let reader = fasta::Reader::from_file(path).unwrap();

        for record in reader.records() {
            let frac = cri::cri(&record.unwrap().seq()); 
            println!("{}", frac);
        }

    }
}
