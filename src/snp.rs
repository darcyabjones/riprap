//! Docstring!

use errors::MyError;

use std::str;
use std::fs::File;
use std::collections::HashMap;
use bio::io::fasta;
use rust_htslib::bcf;
use rust_htslib::bcf::Read;
use rust_htslib::bcf::header::HeaderView;
//use rust_htslib::bcf::Record;


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

pub fn get_chrom_name(rid: Option<u32>, hv: &HeaderView) -> Result<&str, MyError> {
    let rid = rid.ok_or_else(|| {
            MyError::BCFError {desc: String::from("Missing rid encountered.")}
        })?;

    let chrom = hv.rid2name(rid);

    let chrom_str = str::from_utf8(chrom)
        .map_err(|_| {
            MyError::BCFError {
                desc: String::from("Header contains invalid Chrom name.")
            }
        });
    chrom_str
}


pub fn get_chrom<'a>(chrom: &str, genome: &'a HashMap<String, fasta::Record>) -> Result<&'a [u8], MyError> {
    genome.get(chrom)
        .ok_or_else(|| MyError::FastaError {
            desc: String::from("VCF reference Chrom not in Fasta")
        })
        .map(|c| c.seq())
}

//pub fn get_base<'a>(seq: &'a [u8], index: usize) -> Option<&'a [u8]> {
//    let base = 
//}