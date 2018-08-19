extern crate bio;

pub mod freqs;

pub mod runner {

    use std::path::Path;
    use freqs::Frequencies;
    use bio::io::fasta;

    pub fn run(path: &Path) {

        let mut reader = fasta::Reader::from_file(path).unwrap();
        let mut record = fasta::Record::new();

        reader.read(&mut record);

        let mut freq = Frequencies::new(2);
        freq.extend(record.seq().into_iter());
        println!("{:#?}", freq);

        println!("{}", freq.count(&&b'A'));
    }
}
