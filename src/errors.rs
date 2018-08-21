//! Docstring!

//use failure::Error;
use failure::Backtrace;

pub type UnitError<Er> = Result<(), Er>;


#[derive(Debug, Fail)]
pub enum CliError {
    #[fail(display = "Couldn't parse as integer: {}", integer)]
    ParseIntError {
        integer: String,
        backtrace: Backtrace,
    },
    #[fail(display = "Couldn't find file at path: {}", path)]
    PathNotExistError {
        path: String,
        backtrace: Backtrace,
    },
}
