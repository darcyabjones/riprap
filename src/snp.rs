//! Docstring!


use bio::io::fasta;
use rust_htslib::bcf;
use rust_htslib::bcf::header::HeaderView;
use rust_htslib::bcf::record::GenotypeAllele;
use rust_htslib::bcf::Read;
use std::collections::HashMap;
use std::fs::File;
use std::str;
//use rust_htslib::bcf::Record;

use crate::errors::RRError;

pub fn get_samples(reader: &bcf::Reader) -> Vec<&str> {
    reader
        .header()
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
    let rid = rid.ok_or_else(|| MyError::BCFError {
        desc: String::from("Missing rid encountered."),
    })?;

    let chrom = hv.rid2name(rid).unwrap();

    let chrom_str = str::from_utf8(chrom).map_err(|_| MyError::BCFError {
        desc: String::from("Header contains invalid Chrom name."),
    });
    chrom_str
}

pub fn get_genotypes(genotypes: &mut bcf::record::Genotypes, n: u32) -> Vec<Vec<usize>> {
    let mut output: Vec<Vec<usize>> = Vec::new();
    for i in 0..n {
        let i2 = i as usize;
        let geno = genotypes
            .get(i as usize)
            .as_slice()
            .iter()
            .map(|a| a.index())
            .flat_map(|e| e)
            .map(|e| e as usize)
            .collect();
        output.push(geno);
    }
    output
}

pub fn get_chrom<'a>(
    chrom: &str,
    genome: &'a HashMap<String, fasta::Record>,
) -> Result<&'a [u8], MyError> {
    genome
        .get(chrom)
        .ok_or_else(|| MyError::FastaError {
            desc: String::from("VCF reference Chrom not in Fasta"),
        })
        .map(|c| c.seq())
}

pub fn print_bed(chrom: &str, pos: usize, strand: i8, ref_allele: [u8; 2], isrip: bool) {
    print!("{}\t", chrom);
    print!("{}\t", pos);
    print!("{}\t", pos + 1);
    print!("{}\t", strand);
    print!("{}\t", String::from_utf8_lossy(&ref_allele));
    print!("{}", isrip as u8);

    print!("\n");
}
