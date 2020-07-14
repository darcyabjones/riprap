extern crate failure;

mod bedgraph;
//mod errors;
mod runner;
mod snp;
mod stats;

use exitfailure::ExitFailure;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "the stupid content tracker")]
enum Cli {
    GC {
        #[structopt(parse(from_os_str))]
        infile: PathBuf,
        #[structopt(parse(from_os_str), short = "o", long = "output")]
        outfile: Option<PathBuf>,
        #[structopt(short = "w", long = "size", default_value = "5000")]
        window: usize,
        #[structopt(short = "s", long = "step", default_value = "1000")]
        step: usize,
    },
    CRI {
        #[structopt(parse(from_os_str))]
        infile: PathBuf,
        #[structopt(parse(from_os_str), short = "o", long = "output")]
        outfile: Option<PathBuf>,
        #[structopt(short = "w", long = "size", default_value = "5000")]
        window: usize,
        #[structopt(short = "s", long = "step", default_value = "1000")]
        step: usize,
    },
    SNP {
        #[structopt(parse(from_os_str))]
        infile: PathBuf,
        #[structopt(parse(from_os_str))]
        invcf: PathBuf,
        #[structopt(parse(from_os_str), short = "o", long = "output")]
        outfile: Option<PathBuf>,
    },
}

fn main() -> Result<(), ExitFailure> {
    match Cli::from_args() {
        Cli::GC {
            infile,
            outfile,
            window,
            step,
        } => runner::run_gc(&infile, &outfile, window, step)?,
        Cli::CRI {
            infile,
            outfile,
            window,
            step,
        } => runner::run_cri(&infile, &outfile, window, step)?,
        Cli::SNP {
            infile,
            invcf,
            outfile,
        } => runner::run_ripsnp(&infile, &invcf, &outfile)?,
    }

    Ok(())
}

/*
("snp", Some(m)) => {
    SNPConfig::parse_clap(m)
        .and_then(|c| runner::run_ripsnp(&c.fasta, &c.vcf))
},
("", None) => {
    println!("no subcommand");
    Ok(())
},
*/
