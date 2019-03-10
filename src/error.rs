use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct MJCFParseError {
    inner: Context<MJCFParseErrorKind>,
}

#[derive(Clone, PartialEq, Debug, Fail)]
pub enum MJCFParseErrorKind {
    // data contains the displayed roxmltree::Error
    #[fail(display = "{}", 0)]
    BadXML(String),
    #[fail(display = "{}", tag_name)]
    MissingRequiredTag { tag_name: String },
    #[fail(display = "worldbody tag has attributes")]
    WorldBodyHasAttributes,
    #[fail(display = "worldbody has invalid children")]
    WorldBodyInvalidChildren,
    #[fail(display = "invalid geom type {}", geom_type)]
    InvalidGeomType { geom_type: String },
}

impl Fail for MJCFParseError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for MJCFParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl MJCFParseError {
    pub fn kind(&self) -> MJCFParseErrorKind {
        self.inner.get_context().clone()
    }
}

impl From<MJCFParseErrorKind> for MJCFParseError {
    fn from(kind: MJCFParseErrorKind) -> MJCFParseError {
        MJCFParseError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<MJCFParseErrorKind>> for MJCFParseError {
    fn from(inner: Context<MJCFParseErrorKind>) -> MJCFParseError {
        MJCFParseError { inner: inner }
    }
}

pub type MJCFParseResult<T> = Result<T, MJCFParseError>;
