use core::fmt;

pub(crate) type Result<T> = core::result::Result<T, Error>;

/// Error used in wrath, usually originating from imath-sys.
#[derive(Debug, Clone)]
pub enum Error {
    /// When converting from a string representation, the given string contained
    /// a zero-byte that was not at the end.
    IntegerReprContainedNul,
    /// An error occurred when converting a string to an integer value, and the
    /// output was truncated.
    IntegerReprTruncated,
    /// The result of a remainder operation was outside the expected bounds.
    RemainedOutsideBounds,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

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
