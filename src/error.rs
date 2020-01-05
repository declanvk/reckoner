use core::{
    convert::Infallible,
    fmt,
    num::{ParseIntError, TryFromIntError},
};
use std::ffi::CStr;

pub(crate) type Result<T> = core::result::Result<T, Error>;

/// Error used in `reckoner`, usually originating from `imath-sys`.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// When converting from a string representation, the given string contained
    /// a zero-byte that was not at the end.
    StringReprContainedNul,
    /// An error occurred when converting a string to a value, and the
    /// output was truncated.
    ReadStringTruncated,
    /// The result of a remainder operation was outside the expected bounds.
    RemainderOutsideBounds,
    /// Could not convert a value to a primitive integer type because it was
    /// outside the range.
    ConversionOutsideRange,
    /// Integer parse failed.
    IntParseFailed,
    /// It impossible for this error to occur.
    NoErrorPossible,
    /// Unknown value for an imath rounding mode.
    UnknownRoundingMode,
    /// Internal error from `imath_sys`.
    IMath {
        /// Internal `imath_sys` error code.
        code: imath_sys::mp_result,
        /// Custom message to display.
        msg: Option<&'static str>,
    },
    /// The rational value is not a canonical integer, in the form `n/1`.
    NotCanonicalInteger,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::StringReprContainedNul => {
                write!(f, "String representation contained a 'nul' character.")
            }
            Error::ReadStringTruncated => write!(
                f,
                "During conversion, the value representation was truncated."
            ),
            Error::RemainderOutsideBounds => write!(
                f,
                "The result of a remainder operation was outside the expected bounds."
            ),
            Error::ConversionOutsideRange => write!(
                f,
                "Attempted to convert to a primitive integer while outside its valid range."
            ),
            Error::IntParseFailed => write!(f, "Integer parsing failed."),
            Error::UnknownRoundingMode => write!(f, "Unknown value for an imath rounding mode."),
            Error::NoErrorPossible => write!(
                f,
                "This error is not supposed to be possible. Please file an issue."
            ),
            Error::IMath {
                code,
                msg: Some(msg),
            } => write!(
                f,
                "imath error ({:?} \"{}\"): {}",
                code,
                get_imath_sys_error_msg(*code),
                msg
            ),
            Error::IMath { code, msg: None } => write!(
                f,
                "imath error ({:?} \"{}\")",
                code,
                get_imath_sys_error_msg(*code)
            ),
            Error::NotCanonicalInteger => write!(
                f,
                "The rational value is not a canonical integer, in the form `n/1`."
            ),
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

fn get_imath_sys_error_msg(code: imath_sys::mp_result) -> String {
    // This is safe because the function will always return a cstring with static
    // lifetime, even if the error code is not a recognized value.
    let err_char_ptr = unsafe { imath_sys::mp_error_string(code) };

    // This function is safe bc I checked the static string that `mp_error_string`
    // return and they conform to the condition of ending with nul byte. Other
    // conditions around lifetimes, and the borrowed cstring being edited are not an
    // issue because it is converted to a `String` immediately.
    unsafe { CStr::from_ptr(err_char_ptr) }
        .to_string_lossy()
        .into_owned()
}

macro_rules! imath_check_panic {
    ($arg:expr) => {
        // Accessing this is safe bc the MP_OK value is only ever used as an error
        // condition.
        if $arg != unsafe { imath_sys::MP_OK } {
            panic!(
                "{}",
                Error::IMath {
                    code: $arg,
                    msg: None
                }
            );
        }
    };

    ($arg:expr, $msg:tt) => {
        // Accessing this is safe bc the MP_OK value is only ever used as an error
        // condition.
        if $arg != unsafe { imath_sys::MP_OK } {
            panic!(
                "{}",
                Error::IMath {
                    code: $arg,
                    msg: Some($msg)
                }
            );
        }
    };
}
