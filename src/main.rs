extern crate riprap;
extern crate clap;

use riprap::runner;
use clap::{App, SubCommand, Arg};
use std::path::Path;

fn is_usize(i: String) -> Result<(), String> {
    match i.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err("Could not parse as integer".to_string())
    }
}

fn main() {

    let matches = App::new("riprap")
        .version("0.1")
        .author("Darcy Jones <darcy.ab.jones@gmail.com>")
        .about("Tools for finding RIP-like patterns in DNA")
        .subcommand(SubCommand::with_name("gc")
            .about("Calculates a GC% bedgraph")
            .arg(Arg::with_name("infile")
                 .help("The input fasta")
                 .required(true))
            .arg(Arg::with_name("size")
                 .short("w")
                 .long("size")
                 .help("The size of the window")
                 .default_value("5000")
                 .takes_value(true)
                 .validator(is_usize))
            .arg(Arg::with_name("step")
                 .short("s")
                 .long("step")
                 .help("The step")
                 .default_value("1000")
                 .takes_value(true)
                 .validator(is_usize)))
        .subcommand(SubCommand::with_name("cri")
            .about("Calculates a CRI bedgraph")
            .arg(Arg::with_name("infile")
                 .help("The input fasta")
                 .required(true))
            .arg(Arg::with_name("size")
                 .short("w")
                 .long("size")
                 .help("The size of the window")
                 .default_value("5000")
                 .takes_value(true)
                 .validator(is_usize))
            .arg(Arg::with_name("step")
                 .short("s")
                 .long("step")
                 .help("The step")
                 .default_value("1000")
                 .takes_value(true)
                 .validator(is_usize)))
        .subcommand(SubCommand::with_name("snp")
            .about("Find snps that are RIP-like")
            .arg(Arg::with_name("infasta")
                 .help("The input fasta")
                 .required(true))
            .arg(Arg::with_name("invcf")
                 .help("The input vcf")
                 .required(true)))
        .get_matches();

    match matches.subcommand() {
        ("gc", Some(gc_matches)) => {
            let size = gc_matches.value_of("size").unwrap().parse::<usize>().unwrap();
            let step = gc_matches.value_of("step").unwrap().parse::<usize>().unwrap();
            runner::run_gc(Path::new(gc_matches.value_of("infile").unwrap()), size, step)
            },
        ("cri", Some(cri_matches)) => {
            let size = cri_matches.value_of("size").unwrap().parse::<usize>().unwrap();
            let step = cri_matches.value_of("step").unwrap().parse::<usize>().unwrap();
            runner::run_cri(Path::new(cri_matches.value_of("infile").unwrap()), size, step)
            },
        ("snp", Some(snp_matches)) => {
            let fasta = Path::new(snp_matches.value_of("infasta").unwrap());
            let vcf = Path::new(snp_matches.value_of("invcf").unwrap());
            runner::run_ripsnp(fasta, vcf)
            },
        ("", None) => println!("no subcommand"),
        _ => unreachable!(),
    }
}
