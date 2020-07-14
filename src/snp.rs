//! Docstring!

use bio::io::fasta;
use rust_htslib::bcf;
use rust_htslib::bcf::header::HeaderView;
use rust_htslib::bcf::record::GenotypeAllele;
use rust_htslib::bcf::Read;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::str;

use failure::{Error, Fail, ResultExt};
//use rust_htslib::bcf::Record;

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

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
#[fail(display = "Missing RID encountered")]
pub struct MissingRid {}

pub fn get_chrom_name(rid: Option<u32>, hv: &HeaderView) -> Result<&str, Error> {
    let rid = rid.ok_or_else(|| MissingRid {})?;
    let chrom = hv.rid2name(rid).unwrap();

    let chrom_str =
        str::from_utf8(chrom).with_context(|_| format!("Header contains invalid Chrom name."))?;
    Ok(chrom_str)
}

pub fn get_genotypes(genotypes: &mut bcf::record::Genotypes, n: u32) -> Vec<Vec<usize>> {
    let mut output: Vec<Vec<usize>> = Vec::new();
    for i in 0..n {
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

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
#[fail(display = "VCF reference Chrom {} not in Fasta", chrom)]
pub struct FastaMismatch {
    chrom: String,
}

pub fn get_chrom<'a>(
    chrom: &str,
    genome: &'a HashMap<String, fasta::Record>,
) -> Result<&'a [u8], Error> {
    genome.get(chrom).map(|c| c.seq()).ok_or_else(|| {
        FastaMismatch {
            chrom: chrom.to_string(),
        }
        .into()
    })
}

pub fn print_bed(
    handle: &mut impl Write,
    chrom: &str,
    pos: usize,
    strand: i8,
    ref_allele: [u8; 2],
    isrip: bool,
) -> Result<(), std::io::Error> {
    writeln!(
        handle,
        "{}\t{}\t{}\t{}\t{}\t{}",
        chrom,
        pos,
        pos + 1,
        strand,
        String::from_utf8_lossy(&ref_allele),
        isrip as u8,
    )
}
