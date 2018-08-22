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
    use errors::{UnitResult, MyError};
    use std::path::PathBuf;
    use std::str;
    use bio::io::fasta;
    use rust_htslib::bcf;
    use rust_htslib::bcf::Read;


    pub fn run_gc(path: &PathBuf, size: usize, step: usize) -> UnitResult<MyError>{

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

    pub fn run_cri(path: &PathBuf, size: usize, step: usize) -> UnitResult<MyError> {

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

    pub fn run_ripsnp(fasta: &PathBuf, vcf: &PathBuf) -> UnitResult<MyError> {

        let freader = fasta::Reader::from_file(fasta).map_err(|e| {
            MyError::CantReadFileError { path: fasta.to_path_buf(), io_error: e}
        })?;

        let mut breader = bcf::Reader::from_path(vcf).map_err(|e| {
            MyError::BCFError { path: vcf.to_path_buf(), bcf_error: e }
        })?;

        let hv = breader.header().to_owned();

        let genome = snp::fasta_to_dict(freader);
        for record in breader.records() {
            let mut this = record.unwrap();
            println!("pos {:?}", this.pos());
            println!("geno {:?}", this.genotypes().unwrap());
            println!("geno 1 {:?}", this.genotypes().unwrap().get(2));
            let rid = this.rid();
            let alleles = this.alleles().contains(&"A");
            println!("alleles {:?}", alleles);
            let this_pos = this.pos() as usize;

            let chrom = str::from_utf8(rid.map(|x| hv.rid2name(x)).unwrap()).unwrap();
            let seq = genome.get(chrom).unwrap().seq();
            let prev = seq.get(this_pos - 1); // Option
            let next = seq.get(this_pos + 1); // Option

            println!("{:?}", seq.get(this_pos));

            break;
        }

        //let samples = snp::get_samples(&breader);
        //println!("{:?}", samples);

        Ok(())
    }
}
