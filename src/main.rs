extern crate riprap;
extern crate clap;

mod cli;
use riprap::runner;

use std::path::Path;
use std::env;
use std::process;
use std::error;
use std::num;

fn main() {
    let app = cli::build_cli();
    let matches = cli::eval_cli(app, env::args_os());

    let result = match matches.subcommand() {
        ("gc", Some(gc_matches)) => {
            let size = gc_matches
                .value_of("size")
                .map(parse::<usize>)
                .ok_or_else(num::ParseIntError)?;
            let step = gc_matches.value_of("step").unwrap().parse::<usize>().unwrap();
            runner::run_gc(Path::new(gc_matches.value_of("infile").unwrap()), size, step)
            },
        ("cri", Some(cri_matches)) => {
            let size = cri_matches.value_of("size").unwrap().parse::<usize>().unwrap();
            let step = cri_matches.value_of("step").unwrap().parse::<usize>().unwrap();
            runner::run_cri(Path::new(cri_matches.value_of("infile").unwrap()), size, step)
            },
        ("snp", Some(snp_matches)) => {
            let config = cli::SNPSubCommand::parse_clap(snp_matches);
            println!("{:?}", config);
            let fasta = Path::new(snp_matches.value_of("infasta").unwrap());
            let vcf = Path::new(snp_matches.value_of("invcf").unwrap());
            runner::run_ripsnp(fasta, vcf)
            },
        ("", None) => {
            println!("no subcommand");
            Ok(())
        },
        _ => unreachable!(),
    };

    process::exit(match result {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            1
        }
    });
}
