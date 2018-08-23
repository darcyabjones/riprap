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

        // Get the bases first because it's easier to coerce into byte slices.
        let a: u8 = b'A';
        let t: u8 = b'T';
        let g: u8 = b'G';
        let c: u8 = b'C';

        let freader = fasta::Reader::from_file(fasta).map_err(|e| {
            MyError::FastaReadFileError { path: fasta.to_path_buf(), io_error: e}
        })?;

        let mut breader = bcf::Reader::from_path(vcf).map_err(|e| {
            MyError::BCFPathError { path: vcf.to_path_buf(), bcf_error: e }
        })?;

        // Must convert to owned because opened the reader mutably.
        // hv = HeaderView
        let hv = breader.header().to_owned();

        let genome = snp::fasta_to_dict(freader);

        for record in breader.records() {
            let mut this = record.map_err(|err| {
                MyError::BCFReadError {
                    desc: String::from("Error reading vcf record"),
                    bcf_error: err,
                }
            })?;

            let alleles = this.alleles().iter().map(|x| x[0]).collect::<Vec<u8>>();
            let this_base = alleles[0];
            let alt_alleles = &alleles[1..];

            let c_to_t = this_base == c && alt_alleles.contains(&t);
            let t_to_c = this_base == t && alt_alleles.contains(&c);
            let a_to_g = this_base == a && alt_alleles.contains(&g);
            let g_to_a = this_base == g && alt_alleles.contains(&a);


            //println!("{}", c_to_t);
            //println!("{}", t_to_c);
            //println!("{}", g_to_a);
            //println!("{}", a_to_g);

            if !( c_to_t || t_to_c || a_to_g || g_to_a ) {
                continue
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

            if ( c_to_t || t_to_c ) && next_base == a {
                println!(
                    "{}\t{}\t1\t{}\t{}\t{}",
                    chrom,
                    this_pos,
                    String::from_utf8_lossy(&[this_base]),
                    String::from_utf8_lossy(&[next_base]), 
                    String::from_utf8_lossy(&alt_alleles));
            } else if ( g_to_a || a_to_g ) && next_base == t {
                println!(
                    "{}\t{}\t-1\t{}\t{}\t{}",
                    chrom,
                    this_pos,
                    String::from_utf8_lossy(&[this_base]),
                    String::from_utf8_lossy(&[next_base]),
                    String::from_utf8_lossy(&alt_alleles));
            } else {
                //println!("Probably not rip {} {} : alt {:?}", this_base, next_base, alt_alleles);
            }
            //println!("{:?}", next_base == a);
            //println!("{:?}", next_base == t);
            //println!("{:?}", next_base == c);
            //println!("{:?}", next_base == g);
            //println!("{:?}", this_base);
            //println!("{:?}", alt_alleles);

            //println!("next {:?}", next_base);

            //println!("{:?}", c == this_base);
            //println!("{:?}", c == &[prev_base.unwrap()]);
            //break;
        }

        //let samples = snp::get_samples(&breader)
        //println!("{:?}", samples);

        Ok(())
    }
}
