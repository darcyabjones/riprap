//! Docstring!

//use failure::Error;
use std::io;
use std::path;
//use failure::Backtrace;
use rust_htslib::bcf;

pub type UnitResult<Er> = Result<(), Er>;


#[derive(Debug, Fail)]
pub enum MyError {
    #[fail(display = "Couldn't parse as integer: {}", integer)]
    ParseIntError {
        integer: String,
    },
    #[fail(display = "Couldn't find file at path: {}", path)]
    PathNotExistError {
        path: String,
    },
    #[fail(display = "Couldn't read file at path: {:?}", path)]
    CantReadFileError {
        path: path::PathBuf,
        #[cause] io_error: io::Error,
    },
    #[fail(display = "Couldn't read file at path: {:?}", path)]
    BCFError {
        path: path::PathBuf,
        #[cause] bcf_error: bcf::BCFPathError,
    },
}
