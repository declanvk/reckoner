use core::fmt;
use std::error::Error;

pub(crate) type Result<T> = core::result::Result<T, RimathError>;

/// Error used in rimath, usually originating from imath-sys.
#[derive(Debug, Clone)]
pub enum RimathError {
    /// When converting from a string representation, the given string contained
    /// a zero-byte that was not at the end.
    IntegerReprContainedNul,
    /// An error occurred when converting a string to an integer value, and the
    /// output was truncated.
    IntegerReprTruncated,
    /// The result of a remainder operation was outside the expected bounds.
    RemainedOutsideBounds,
}

impl Error for RimathError {}

impl fmt::Display for RimathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RimathError::*;

        match self {
            IntegerReprContainedNul => {
                write!(f, "Integer representation contained a 'nul' character.")
            }
            IntegerReprTruncated => write!(
                f,
                "During conversion, the integer conversion was not read fully."
            ),
            RemainedOutsideBounds => write!(
                f,
                "The result of a remainder operation was outside the expected bounds."
            ),
        }
    }
}
