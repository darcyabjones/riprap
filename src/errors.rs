//! Docstring!

//use failure::Error;
use std::io;
use std::path;
//use failure::Backtrace;
use rust_htslib::bcf;

pub type UnitResult<Er> = Result<(), Er>;

#[derive(Debug, Fail)]
pub enum MyError {
    #[fail(
        display = "Required input not provided. If you encounter this error, please submit a bug report."
    )]
    RequiredInputMissing,
    #[fail(display = "Couldn't parse as integer: {}", integer)]
    ParseIntError { integer: String },
    #[fail(display = "Couldn't find file at path: {}", path)]
    PathNotExistError { path: String },
    #[fail(display = "Couldn't read file at path: {:?}", path)]
    FastaReadFileError {
        path: path::PathBuf,
        #[cause]
        io_error: io::Error,
    },
    #[fail(display = "Error with Fasta: {}", desc)]
    FastaError { desc: String },
    #[fail(display = "Couldn't read file at path: {:?}", path)]
    BCFPathError {
        path: path::PathBuf,
        #[cause]
        bcf_error: bcf::BCFPathError,
    },
    #[fail(display = "Error Processing VCF: {}", desc)]
    BCFError { desc: String },
    #[fail(display = "Error Processing VCF: {}", desc)]
    BCFReadError {
        desc: String,
        #[cause]
        bcf_error: bcf::ReadError,
    },
    #[fail(display = "Error Processing VCF: {}", desc)]
    BCFFormatReadError {
        desc: String,
        #[cause]
        bcf_error: bcf::record::FormatReadError,
    },
}
