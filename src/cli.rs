//! # cli
//!
//! The `cli` module contains structs and trait implementations for
//! parsing command line arguments.

use riprap::errors::MyError;

use std::env;
use std::path::PathBuf;

use clap::{App, Arg, ArgMatches, SubCommand};

/// Parse a string as integer raising an error if invalid or None.
fn parse_usize(i: Option<&str>) -> Result<usize, MyError> {
    let j = i.ok_or_else(|| MyError::RequiredInputMissing)?;

    match j.parse::<usize>() {
        Ok(k) => Ok(k),
        Err(_) => Err(MyError::ParseIntError {
            integer: j.to_string(),
        }),
    }
}

/// Helper function to check if can parse as int.
fn is_usize(i: String) -> Result<(), String> {
    match parse_usize(Some(&i)) {
        Ok(_) => Ok(()),
        Err(_) => Err("Could not parse as integer".to_string()),
    }
}

/// Parse a string as a file path, raising error if None, or doesn't exist.
fn parse_file(path: Option<&str>) -> Result<PathBuf, MyError> {
    let spath = path.ok_or_else(|| MyError::RequiredInputMissing)?;

    // If stdin or stdout
    if spath == "-" {
        return Ok(PathBuf::from(spath));
    }

    let pb = PathBuf::from(spath);

    if pb.is_file() {
        Ok(pb)
    } else {
        Err(MyError::PathNotExistError {
            path: spath.to_string(),
        })
    }
}

/// Helper function to check if file exists.
fn is_file(path: String) -> Result<(), String> {
    match parse_file(Some(&path)) {
        Ok(_) => Ok(()),
        Err(_) => Err("File does not exist or isn't regular file.".to_string()),
    }
}

/// Arguments for the main riprap cli.
pub fn build_cli() -> App<'static, 'static> {
    App::new("riprap")
        .version("0.1")
        .author("Darcy Jones <darcy.ab.jones@gmail.com>")
        .about("Tools for finding RIP-like patterns in DNA")
        .subcommand(cli_sub_sliding("gc", "Calculate GC% in a sliding window."))
        .subcommand(cli_sub_sliding("cri", "Calculate CRI in a sliding window."))
        .subcommand(cli_sub_snp())
}

/// Arguments for the sliding window family of subcommands.
/// This allows us to use the same config for GC and CRI windows.
pub fn cli_sub_sliding(name: &'static str, about: &'static str) -> App<'static, 'static> {
    SubCommand::with_name(name)
        .about(about)
        .arg(
            Arg::with_name("fasta")
                .help("The reference fasta to calculate windows over. Use '-' for stdin.")
                .index(1)
                .required(true)
                .validator(is_file),
        )
        .arg(
            Arg::with_name("window")
                .short("w")
                .long("size")
                .help("The size of the window")
                .default_value("5000")
                .takes_value(true)
                .validator(is_usize),
        )
        .arg(
            Arg::with_name("step")
                .short("s")
                .long("step")
                .help("The step")
                .default_value("1000")
                .takes_value(true)
                .validator(is_usize),
        )
}

/// Arguments for the SNP subcommand
pub fn cli_sub_snp() -> App<'static, 'static> {
    SubCommand::with_name("snp")
        .about("Find snps that are RIP-like")
        .arg(Arg::with_name("infasta")
             .help("The reference fasta. Use '-' for stdin.")
             .required(true)
             .validator(is_file))
        .arg(Arg::with_name("invcf")
             .help("The genotyped vcf. GZIPped files will be automatically unzipped. Use '-' for stdin.")
             .required(true)
             .validator(is_file))
}

/// Get the actual provided arguments given the cli and argv.
pub fn eval_cli(app: App<'static, 'static>, args: env::ArgsOs) -> ArgMatches<'static> {
    app.get_matches_from(args)
}

/// A trait for parsing clap arguments into our own structs.
/// We also give the opportunity for parsing to fail, so
/// return a Result.
pub trait Config {
    fn parse_clap(app: &ArgMatches<'static>) -> Result<Box<Self>, MyError>;
}

/// The config struct for windowed CLI subcommands.
#[derive(Debug)]
pub struct WindowConfig {
    pub fasta: PathBuf,
    pub window: usize,
    pub step: usize,
}

impl Config for WindowConfig {
    /// Parse provided argument matches to our structure.
    /// Raising errors if incorrect args provided.
    fn parse_clap(app: &ArgMatches<'static>) -> Result<Box<Self>, MyError> {
        let fasta = parse_file(app.value_of("fasta"))?;
        let window = parse_usize(app.value_of("window"))?;
        let step = parse_usize(app.value_of("step"))?;
        let config = Self {
            fasta: fasta,
            window: window,
            step: step,
        };
        Ok(Box::new(config))
    }
}

/// The config struct for the SNP cli subcommand
#[derive(Debug)]
pub struct SNPConfig {
    pub fasta: PathBuf,
    pub vcf: PathBuf,
}

impl Config for SNPConfig {
    /// Parse provided argument matches to our structure.
    /// Raising errors if incorrect args provided.
    fn parse_clap(app: &ArgMatches<'static>) -> Result<Box<Self>, MyError> {
        let fasta = parse_file(app.value_of("infasta"))?;
        let vcf = parse_file(app.value_of("invcf"))?;
        let config = Self {
            fasta: fasta,
            vcf: vcf,
        };
        Ok(Box::new(config))
    }
}
