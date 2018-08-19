extern crate riprap;
extern crate clap;

use riprap::runner;
use clap::{App, SubCommand, Arg};
use std::path::Path;

fn main() {

    let matches = App::new("riprap")
        .version("0.1")
        .author("Darcy Jones <darcy.ab.jones@gmail.com>")
        .about("Tools for finding RIP-like patterns in DNA")
        .arg(Arg::with_name("fasta")
             .short("i")
             .long("infile")
             .value_name("FILE")
             .help("input file")
             .takes_value(true))
        .get_matches();

    let fasta = matches.value_of("fasta").unwrap_or("default");
    println!("{}", fasta);
    runner::run(Path::new(fasta))
}
