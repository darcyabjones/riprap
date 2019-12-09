//! # cli
//!
//! The `cli` module contains structs and trait implementations for
//! parsing command line arguments.

use std::path::PathBuf;

use structopt::StructOpt;
use clap::crate_description;
use snafu::Snafu;


/// Utility for displaying lists in error messages.
fn vec_as_comma_delimited(v: &[String]) -> String {
    v.join(", ")
}


#[derive(Debug, StructOpt)]
#[structopt(about = crate_description!())]
pub(crate) struct Options {

    /// Write progress to screen during runtime.
    /// Specify multiple times to increase verbosity (i.e. debug level).
    #[structopt(short, long, parse(from_occurrences))]
    pub(crate) verbose: u8,

    #[structopt(subcommand)]
    pub(crate) cmd: SubCommand,

}


#[derive(Debug, StructOpt)]
pub(crate) enum SubCommand {

    /// Calculate base frequency statistics in a rolling window.
    Window {
        /// The fields to print in the output bedgraph.
        #[structopt(short, long, default_value = "all")]
        fields: Vec<WindowField>,
        /// The size of the sliding window.
        #[structopt(short = "w", long = "window-size", default_value = "5000")]
        size: usize,
        /// The step of the sliding window.
        #[structopt(short = "s", long = "window-step", default_value = "1000")]
        step: usize,
        /// Where to write the output bedgraph. Use '-' for stdout.
        #[structopt(short, long, parse(from_os_str), default_value = "-")]
        output: PathBuf,
        /// The input fasta file. Use '-' for stdin.
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    }
}

/// Error kinds for CLI issues.
#[derive(Debug, Snafu)]
pub(crate) enum Error {
    #[snafu(display(
        "Received an invalid choice: {}. Valid choices are: {}",
        bad_choice,
        vec_as_comma_delimited(valid_choices)
    ))]
    ChoiceError { bad_choice: String, valid_choices: Vec<String> },
}


// Enum types for controlling options.

/// Valid choices for the window subcommand fields option.
/// Some options are short hand for multiple options.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub(crate) enum WindowField {
    PercGC,
    CRI,
    Margolin1,
    Margolin2,
    Di,
    Tri,
    DiNR,
    TriNR,
    All,
}


impl WindowField {

    fn domain() -> Vec<String> {
        vec!["gc".to_string(), "cri".to_string(), "all".to_string()]
    }

    fn expand(&self) -> Vec<> {
        window::Field
    }

}



impl std::str::FromStr for WindowField {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "gc" => Ok(WindowField::GC),
            "cri" => Ok(WindowField::CRI),
            "all" => Ok(WindowField::All),
            m => Err(
                Error::ChoiceError {
                    bad_choice: m.to_string(),
                    valid_choices: WindowField::domain()
                }
            ),
        }
    }

}
