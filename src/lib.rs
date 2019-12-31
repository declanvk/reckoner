#![deny(missing_docs)]

//! Arbitrary precision integer and rational arithmetic library wrapping
//! [`imath`](https://github.com/creachadair/imath/)

#[macro_use]
pub(crate) mod error;
pub(crate) mod integer;
pub(crate) mod rational;

pub use crate::{
    error::Error,
    integer::Integer,
    rational::{Rational, RoundMode},
};
