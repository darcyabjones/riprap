extern crate bio;
extern crate rust_htslib;

pub mod freqs;
pub mod stats;

pub mod snp {
    use rust_htslib::bcf;


}

pub mod runner {

    use std::path::Path;
    use bio::io::fasta;
    use rust_htslib::bcf;
    use rust_htslib::bcf::Read;
    use rust_htslib::bcf::Record;
    use stats;
    use std::str;

    pub fn run_gc(path: &Path, size: usize, step: usize) {

        let reader = fasta::Reader::from_file(path).unwrap();

        for record in reader.records() {
            let rec = record.unwrap();
            let frac = stats::run_sliding_windows(&rec, size, step, stats::gc_content); 
            for (seqid, start, end, score) in frac {
                println!("{}\t{}\t{}\t{}", seqid, start, end, score);
            }
        }

    }

    pub fn run_cri(path: &Path, size: usize, step: usize) {

        let reader = fasta::Reader::from_file(path).unwrap();

        for record in reader.records() {
            let rec = record.unwrap();
            let frac = stats::run_sliding_windows(&rec, size, step, stats::cri); 
            for (seqid, start, end, score) in frac {
                println!("{}\t{}\t{}\t{}", seqid, start, end, score);
            }
        }

    }

    pub fn run_ripsnp(fasta: &Path, vcf: &Path) {
        let freader = fasta::Reader::from_file(fasta).unwrap();
        let mut breader = bcf::Reader::from_path(vcf).unwrap();

        let samples: Vec<&str> = breader.header().samples().iter().map(|x| str::from_utf8(x).unwrap()).cloned().collect();

        for record in breader.records() {
            let mut this = record.unwrap();
            println!("id {:?}", this.id());
            println!("pos {:?}", this.pos());
            println!("alleles {:?}", this.alleles());
            println!("geno {:?}", this.genotypes().unwrap());
            println!("geno 1 {:?}", this.genotypes().unwrap().get(2));
            break;
        }
    }
}
