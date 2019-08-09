//! # Runner module
//!
//! `runner` is module containing high-level pipeline functions

use bio::io::fasta;
use std::path::PathBuf;

use crate::errors::UnitResult;
use crate::stats;
//use crate::snp;

pub fn run_gc(
    path: &PathBuf,
    outfile: &PathBuf,
    size: usize,
    step: usize
) -> UnitResult {
    let reader = fasta::Reader::from_file(path).unwrap();

    for record in reader.records() {
        let rec = record.unwrap();
        let frac = stats::sliding_windows(
            &rec,
            size,
            step,
            |x| stats::base_content(x, b"GC")
        );

        for rec in frac {
            println!("{}", rec);
        }
    }
    Ok(())
}

pub fn run_cri(
    path: &PathBuf,
    outfile: &PathBuf,
    size: usize,
    step: usize
) -> UnitResult {
    let reader = fasta::Reader::from_file(path).unwrap();

    for record in reader.records() {
        let rec = record.unwrap();
        let frac = stats::sliding_windows(&rec, size, step, stats::cri);
        for rec in frac {
            println!("{}", rec);
        }
    }
    Ok(())
}

/*
pub fn run_ripsnp(fasta: &PathBuf, vcf: &PathBuf) -> UnitResult {
    // Get the bases first because it's easier to coerce into byte slices.
    let a: u8 = b'A';
    let t: u8 = b'T';
    let g: u8 = b'G';
    let c: u8 = b'C';

    let freader = fasta::Reader::from_file(fasta)
        .map_err(|e| e.context(RRErrorKind::FastaReadFileError { path: fasta.to_path_buf() })?;

    let mut breader = bcf::Reader::from_path(vcf)
        .map_err(|e| e.context(RRErrorKind::BCFPathError { path: vcf.to_path_buf() } )?;

    // Must convert to owned because opened the reader mutably.
    // hv = HeaderView
    let hv = breader.header().to_owned();
    let genome = snp::fasta_to_dict(freader);

    for record in breader.records() {
        let mut this = record
            .map_err(|e| e.context(RRErrorKind::BCFReadError { desc: String::from("Error reading vcf record") })?;

        let alleles = this.alleles().to_owned();

        let ref_allele = &alleles[0];
        if ref_allele.len() > 1 {
            continue;
        }

        let this_base = ref_allele[0] as u8;
        let alt_alleles = &alleles[1..]
            .iter()
            .filter(|x| x.len() == 1)
            .map(|x| x[0])
            .collect::<Vec<u8>>();

        if alt_alleles.len() == 0 {
            continue;
        }

        let c_to_t = this_base == c && alt_alleles.contains(&t);
        let t_to_c = this_base == t && alt_alleles.contains(&c);
        let a_to_g = this_base == a && alt_alleles.contains(&g);
        let g_to_a = this_base == g && alt_alleles.contains(&a);

        if !(c_to_t || t_to_c || a_to_g || g_to_a) {
            continue;
        }

        let chrom = snp::get_chrom_name(this.rid(), &hv)?;
        let seq = snp::get_chrom(chrom, &genome)?;

        let this_pos = this.pos() as usize;
        let next_pos = if c_to_t || t_to_c {
            this_pos + 1
        } else {
            this_pos - 1
        };

        let next_base = match seq.get(next_pos) {
            Some(x) => *x,
            None => {
                continue;
                unreachable!() // This isn't ideal but I don't have any better
            }
        };

        let mut strand: i8 = 0;
        let mut isrip = false;

        if (c_to_t || t_to_c) && next_base == a {
            strand = 1;
            isrip = true;
        } else if (g_to_a || a_to_g) && next_base == t {
            strand = -1;
            isrip = true;
        } else {
            isrip = false;
        }

        //let mut genotypes = &this.genotypes().clone().map_err(|err| {
        //    MyError::BCFFormatReadError {
        //        desc: String::from("Genotype fields were malformed"),
        //        bcf_error: err,
        //    }
        //})?.to_owned();

        //let g = snp::get_genotypes(&mut genotypes, 144);
        //println!("{:?}", g);

        snp::print_bed(chrom, this_pos, strand, [this_base, next_base], isrip);
    }

    //let samples = snp::get_samples(&breader)
    //println!("{:?}", samples);

    Ok(())
}

*/
