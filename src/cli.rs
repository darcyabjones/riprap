use std::error::Error;
use std::env;
use std::path;
use std::path::Path;
use std::path::PathBuf;

use clap::{App, SubCommand, Arg, ArgMatches};


fn is_usize(i: String) -> Result<(), String> {
    match i.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err("Could not parse as integer".to_string())
    }
}

pub struct GCSubCommand {
    pub fasta: PathBuf,
    pub window: usize,
    pub step: usize,
}

impl GCSubCommand {
    pub fn new(fasta: &Path, window: usize, step: usize) -> Self {
        GCSubCommand {
            fasta: PathBuf::from(fasta),
            window: window,
            step: step,
        }
    }

    pub fn clap() ->  App<'static, 'static> {
        SubCommand::with_name("gc")
            .about("Calculates a GC% bedgraph")
            .arg(Arg::with_name("fasta")
                 .help("The input fasta")
                 .required(true))
            .arg(Arg::with_name("window")
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
                 .validator(is_usize))
    }

    pub fn parse_clap(app: &ArgMatches<'static>) -> Result<Self, Box<Error>> {
        let fasta = app.value_of("fasta").unwrap();
        let window = app.value_of("size")
            .map(parse::<usize>)
            .ok_or_else(num::ParseIntError)?;
        let step = app.value_of("step")
            .map(parse::<usize>)
            .ok_or_else(num::ParseIntError)?;
        let config = Self::new(Path::new(fasta), window, step);
        Ok(config)
    }
}

pub struct CRISubCommand {
    pub fasta: PathBuf,
    pub window: usize,
    pub step: usize,
}

impl CRISubCommand {
    pub fn new(fasta: &Path, window: usize, step: usize) -> Self {
        CRISubCommand {
            fasta: PathBuf::from(fasta),
            window: window,
            step: step,
        }
    }

    pub fn clap() -> App<'static, 'static> {
        SubCommand::with_name("cri")
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
                 .validator(is_usize))
    }
}

#[derive(Debug)]
pub struct SNPSubCommand {
    pub fasta: PathBuf,
    pub vcf: PathBuf,
}

impl SNPSubCommand {
    pub fn new(fasta: &Path, vcf: &Path) -> Self {
        SNPSubCommand {
            fasta: PathBuf::from(fasta),
            vcf: PathBuf::from(vcf),
        }
    }

    pub fn clap() -> App<'static, 'static> {
        SubCommand::with_name("snp")
            .about("Find snps that are RIP-like")
            .arg(Arg::with_name("infasta")
                 .help("The input fasta")
                 .required(true))
            .arg(Arg::with_name("invcf")
                 .help("The input vcf")
                 .required(true))
    }

    pub fn parse_clap(app: &ArgMatches<'static>) -> Result<Self, Box<Error>> {
        let fasta = app.value_of("infasta").unwrap();
        let vcf = app.value_of("invcf").unwrap();
        let config = Self::new(Path::new(fasta), Path::new(vcf));
        Ok(config)
    }
}


pub fn build_cli() -> App<'static, 'static> {
    App::new("riprap")
        .version("0.1")
        .author("Darcy Jones <darcy.ab.jones@gmail.com>")
        .about("Tools for finding RIP-like patterns in DNA")
        .subcommand(GCSubCommand::clap())
        .subcommand(CRISubCommand::clap())
        .subcommand(SNPSubCommand::clap())
}

pub fn eval_cli(app: App<'static, 'static>, args: env::ArgsOs) -> ArgMatches<'static> {
    app.get_matches_from(args)
}
