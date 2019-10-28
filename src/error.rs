use core::fmt;
use std::error::Error;

pub(crate) type Result<T> = core::result::Result<T, RimathError>;

#[derive(Debug, Clone)]
pub enum RimathError {
    IntegerReprContainedNul,
    IntegerReprTruncated,
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
        }
    }
}
