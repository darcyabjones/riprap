//! Docstring!

use std::str;
use std::fs::File;
use std::collections::HashMap;
use bio::io::fasta;
use rust_htslib::bcf;
use rust_htslib::bcf::Read;
use rust_htslib::bcf::Record;


pub fn get_samples(reader: &bcf::Reader) -> Vec<&str> {
    reader.header()
        .samples()
        .iter()
        .map(|x| str::from_utf8(x).unwrap())
        .collect()
}


pub fn fasta_to_dict(records: fasta::Reader<File>) -> HashMap<String, fasta::Record> {
    let mut output = HashMap::new();

    for rec in records.records() {
        let rec2 = &rec.unwrap();
        output.insert(rec2.id().to_owned(), rec2.to_owned());
    }
    output
}
