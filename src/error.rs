use core::{
    convert::Infallible,
    fmt,
    num::{ParseIntError, TryFromIntError},
};

pub(crate) type Result<T> = core::result::Result<T, Error>;

/// Error used in `reckoner`, usually originating from `imath-sys`.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// When converting from a string representation, the given string contained
    /// a zero-byte that was not at the end.
    IntegerReprContainedNul,
    /// An error occurred when converting a string to an integer value, and the
    /// output was truncated.
    IntegerReprTruncated,
    /// The result of a remainder operation was outside the expected bounds.
    RemainderOutsideBounds,
    /// Could not convert a value to a primitive integer type because it was
    /// outside the range.
    ConversionOutsideRange,
    /// Integer parse failed.
    IntParseFailed,
    /// It impossible for this error to occur.
    NoErrorPossible,
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
            RemainderOutsideBounds => write!(
                f,
                "The result of a remainder operation was outside the expected bounds."
            ),
            ConversionOutsideRange => write!(
                f,
                "Attempted to convert to a primitive integer while outside its valid range."
            ),
            IntParseFailed => write!(f, "Integer parsing failed."),
            NoErrorPossible => {
                panic!("This error is no supposed to be possible. Please file an issue.")
            }
        }
    }
}

impl From<TryFromIntError> for Error {
    fn from(_src: TryFromIntError) -> Self {
        Error::ConversionOutsideRange
    }
}

impl From<Infallible> for Error {
    fn from(_src: Infallible) -> Self {
        Error::NoErrorPossible
    }
}

impl From<ParseIntError> for Error {
    fn from(_src: ParseIntError) -> Self {
        Error::IntParseFailed
    }
}
