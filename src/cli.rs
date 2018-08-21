use riprap::errors::CliError;

use std::env;
use std::path::PathBuf;

use clap::{App, SubCommand, Arg, ArgMatches};


fn is_usize(i: String) -> Result<(), String> {
    match i.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err("Could not parse as integer".to_string())
    }
}

pub trait Config {
    fn parse_clap(app: &ArgMatches<'static>) -> Result<Box<Self>, CliError>;
}

#[derive(Debug)]
pub struct WindowConfig {
    pub fasta: PathBuf,
    pub window: usize,
    pub step: usize,
}

#[derive(Debug)]
pub struct SNPConfig {
    pub fasta: PathBuf,
    pub vcf: PathBuf,
}

impl Config for WindowConfig {
    fn parse_clap(app: &ArgMatches<'static>) -> Result<Box<Self>, CliError> {
        let fasta = app.value_of("fasta").unwrap();
        let window = app.value_of("size").unwrap().parse::<usize>().unwrap();
        let step = app.value_of("step").unwrap().parse::<usize>().unwrap();
        let config = Self {fasta: PathBuf::from(fasta), window: window, step: step};
        Ok(Box::new(config))
    }
}

impl Config for SNPConfig {
    fn parse_clap(app: &ArgMatches<'static>) -> Result<Box<Self>, CliError> {
        let fasta = app.value_of("infasta").unwrap();
        let vcf = app.value_of("invcf").unwrap();
        let config = Self { fasta: PathBuf::from(fasta), vcf: PathBuf::from(vcf) };
        Ok(Box::new(config))
    }
}


pub fn build_cli() -> App<'static, 'static> {
    App::new("riprap")
        .version("0.1")
        .author("Darcy Jones <darcy.ab.jones@gmail.com>")
        .about("Tools for finding RIP-like patterns in DNA")
        .subcommand(cli_sub_sliding("gc", "Calculate GC% in a sliding window."))
        .subcommand(cli_sub_sliding("cri", "Calculate CRI in a sliding window."))
        .subcommand(cli_sub_snp())
}

pub fn cli_sub_sliding(name: &'static str, about: &'static str) ->  App<'static, 'static> {
    SubCommand::with_name(name)
        .about(about)
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


pub fn cli_sub_snp() -> App<'static, 'static> {
    SubCommand::with_name("snp")
        .about("Find snps that are RIP-like")
        .arg(Arg::with_name("infasta")
             .help("The input fasta")
             .required(true))
        .arg(Arg::with_name("invcf")
             .help("The input vcf")
             .required(true))
    }

pub fn eval_cli(app: App<'static, 'static>, args: env::ArgsOs) -> ArgMatches<'static> {
    app.get_matches_from(args)
}
