extern crate bio;
extern crate rust_htslib;
#[macro_use] extern crate failure;

pub mod freqs;
pub mod stats;
pub mod errors;
pub mod snp;


pub mod runner {
    //! Docstring!

    use stats;
    use snp;
    use errors::{UnitError, CliError};
    use std::path::PathBuf;
    use bio::io::fasta;
    use rust_htslib::bcf;
    use rust_htslib::bcf::Read;


    pub fn run_gc(path: &PathBuf, size: usize, step: usize) -> UnitError<CliError>{

        let reader = fasta::Reader::from_file(path).unwrap();

        for record in reader.records() {
            let rec = record.unwrap();
            let frac = stats::run_sliding_windows(&rec, size, step, stats::gc_content);
            for (seqid, start, end, score) in frac {
                println!("{}\t{}\t{}\t{}", seqid, start, end, score);
            }
        }
        Ok(())
    }

    pub fn run_cri(path: &PathBuf, size: usize, step: usize) -> UnitError<CliError> {

        let reader = fasta::Reader::from_file(path).unwrap();

        for record in reader.records() {
            let rec = record.unwrap();
            let frac = stats::run_sliding_windows(&rec, size, step, stats::cri);
            for (seqid, start, end, score) in frac {
                println!("{}\t{}\t{}\t{}", seqid, start, end, score);
            }
        }
        Ok(())
    }

    pub fn run_ripsnp(fasta: &PathBuf, vcf: &PathBuf) -> UnitError<CliError> {
        let freader = fasta::Reader::from_file(fasta).unwrap();
        let mut breader = bcf::Reader::from_path(vcf).unwrap();

        for record in breader.records() {
            let mut this = record.unwrap();
            println!("id {:?}", this.id());
            println!("pos {:?}", this.pos());
            println!("alleles {:?}", this.alleles());
            println!("geno {:?}", this.genotypes().unwrap());
            println!("geno 1 {:?}", this.genotypes().unwrap().get(2));
            break;
        }
        let samples = snp::get_samples(&breader);
        println!("{:?}", samples);
        let genome = snp::fasta_to_dict(freader);
        println!("{:?}", genome);
        Ok(())
    }
}
