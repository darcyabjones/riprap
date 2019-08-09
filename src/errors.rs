//! Error types for the library and executable.

use failure::{Backtrace, Context, Error, Fail};
use std::fmt;
use std::fmt::Display;
use std::path;


pub type UnitResult = Result<(), ErrorKind>;

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Required input not provided.")]
    RequiredInputMissing,
    #[fail(display = "Couldn't parse as integer: {}", _0)]
    ParseIntError(String),
    #[fail(display = "Couldn't find file at path: {}", _0)]
    PathNotExistError(String),
    #[fail(display = "Couldn't read file at path: {:?}", _0)]
    FastaReadFileError(path::PathBuf),
    #[fail(display = "Error with Fasta: {}", _0)]
    FastaError(String),
}


impl ErrorKind {

    pub fn ecode(&self) -> i32 {
        match self {
            ErrorKind::RequiredInputMissing => 1,
            ErrorKind::ParseIntError(_) => 1,
            ErrorKind::PathNotExistError(_) => 1,
            ErrorKind::FastaReadFileError(_) => 1,
            ErrorKind::FastaError(_) => 1,
        }
    }

    pub fn pretty_error(&self) -> String {
        self.to_string()
    }
}


//    #[fail(display = "Couldn't read file at path: {:?}", path)]
//    BCFPathError { path: path::PathBuf },
//    #[fail(display = "Error Processing VCF: {}", desc)]
//    BCFError { desc: String },
//    #[fail(display = "Error Processing VCF: {}", desc)]
//    BCFReadError { desc: String },
//    #[fail(display = "Error Processing VCF: {}", desc)]
//    BCFFormatReadError { desc: String },
