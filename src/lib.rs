#![deny(missing_docs)]

//! Arbitrary precision integer and rational arithmetic library wrapping
//! (imath)[https://github.com/creachadair/imath/]

pub(crate) mod error;
pub(crate) mod integer;

pub use crate::{error::RimathError, integer::Integer};
