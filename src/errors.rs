//! Error types for the library and executable.

use std::io;
use std::path;
use std::fmt;
use std::fmt::Display;
use failure::{Backtrace, Context, Fail, Error};
use rust_htslib::bcf;

#[derive(Debug)]
pub struct RRError {
    inner: Context<RRErrorKind>,
}

impl Fail for RRError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for RRError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl RRError {
    pub fn kind(&self) -> RRErrorKind {
        *self.inner.get_context()
    }
}

impl From<RRErrorKind> for RRError {
    fn from(kind: RRErrorKind) -> Self {
        Self { inner: Context::new(kind) }
    }
}

impl From<Context<RRErrorKind>> for RRError {
    fn from(inner: Context<RRErrorKind>) -> Self {
        Self { inner: inner }
    }
}

pub type RRUnitResult = Result<(), RRError>;

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum RRErrorKind {
    #[fail(display = "Required input not provided.")]
    RequiredInputMissing,
    #[fail(display = "Couldn't parse as integer: {}", integer)]
    ParseIntError { integer: String },
    #[fail(display = "Couldn't find file at path: {}", path)]
    PathNotExistError { path: String },
    #[fail(display = "Couldn't read file at path: {:?}", path)]
    FastaReadFileError { path: path::PathBuf },
    #[fail(display = "Error with Fasta: {}", desc)]
    FastaError { desc: String },
    #[fail(display = "Couldn't read file at path: {:?}", path)]
    BCFPathError { path: path::PathBuf },
    #[fail(display = "Error Processing VCF: {}", desc)]
    BCFError { desc: String },
    #[fail(display = "Error Processing VCF: {}", desc)]
    BCFReadError { desc: String },
    #[fail(display = "Error Processing VCF: {}", desc)]
    BCFFormatReadError { desc: String },
}
